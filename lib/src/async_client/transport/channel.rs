use std::{pin::Pin, str::FromStr, sync::Arc, time::Duration};

use arc_swap::ArcSwap;
use futures::{Future, FutureExt};
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::{
    async_client::session::SessionInfo,
    client::prelude::{
        ByteString, CertificateStore, CloseSecureChannelRequest, NodeId, RequestHeader, Role,
        SecureChannel, SecurityPolicy, SecurityTokenRequestType, StatusCode, SupportedMessage,
    },
    sync::RwLock,
    types::DecodingOptions,
};

use super::state::{Request, RequestSend, SecureChannelState, State};

use crate::async_client::{
    retry::SessionRetryPolicy,
    transport::{
        tcp::{TcpTransport, TransportConfiguration},
        OutgoingMessage,
    },
};

/// Wrapper around an open secure channel
pub struct AsyncSecureChannel {
    session_info: SessionInfo,
    session_retry_policy: SessionRetryPolicy,
    pub(crate) secure_channel: Arc<RwLock<SecureChannel>>,
    certificate_store: Arc<RwLock<CertificateStore>>,
    transport_config: TransportConfiguration,
    /// Ignore clock skew between the client and the server.
    run_transport_in_parallel: bool,
    state: SecureChannelState,

    state_watch_rx: tokio::sync::watch::Receiver<State>,
    state_watch_tx: tokio::sync::watch::Sender<State>,
    token: CancellationToken,
}

impl AsyncSecureChannel {
    pub fn new(
        certificate_store: Arc<RwLock<CertificateStore>>,
        session_info: SessionInfo,
        session_retry_policy: SessionRetryPolicy,
        decoding_options: DecodingOptions,
        ignore_clock_skew: bool,
        run_transport_in_parallel: bool,
        auth_token: Arc<ArcSwap<NodeId>>,
    ) -> Self {
        let secure_channel = Arc::new(RwLock::new(SecureChannel::new(
            certificate_store.clone(),
            Role::Client,
            decoding_options,
        )));

        let (state_watch_tx, state_watch_rx) = tokio::sync::watch::channel(State::Disconnected);

        Self {
            transport_config: TransportConfiguration {
                max_pending_incoming: 5,
                max_inflight: 5,
                send_buffer_size: 65535,
                recv_buffer_size: 65535,
                max_message_size: 65535,
                max_chunk_count: 5,
            },
            state: SecureChannelState::new(ignore_clock_skew, secure_channel.clone(), auth_token),
            session_info,
            secure_channel,
            certificate_store,
            session_retry_policy,
            run_transport_in_parallel,
            state_watch_rx,
            state_watch_tx,
            token: CancellationToken::new(),
        }
    }

    pub async fn run(&self) -> Result<(), StatusCode> {
        {
            if !matches!(*self.state_watch_rx.borrow(), State::Disconnected) {
                error!("Secure channel is already running");
                return Err(StatusCode::BadInvalidState);
            }
            if self.token.is_cancelled() {
                error!("Secure channel has already been cancelled");
                return Err(StatusCode::BadInvalidState);
            }
            let _ = self.state_watch_tx.send(State::Connecting);
        }

        let res = self.run_inner().await;

        let _ = self.state_watch_tx.send(State::Disconnected);

        res
    }

    async fn wait_for_state(&self, connected: bool) -> bool {
        let mut rx = self.state_watch_rx.clone();

        loop {
            if !rx.changed().await.is_ok() {
                return false;
            };
            {
                let state = rx.borrow();
                if connected && matches!(*state, State::Connected(_))
                    || !connected && matches!(*state, State::Disconnected)
                {
                    return true;
                }
            }
        }
    }

    pub async fn wait_for_connection(&self) -> bool {
        self.wait_for_state(true).await
    }

    pub async fn send(
        &self,
        request: impl Into<SupportedMessage>,
        timeout: Duration,
    ) -> Result<SupportedMessage, StatusCode> {
        let send = {
            if let State::Connected(s) = &*self.state_watch_rx.borrow() {
                s.clone()
            } else {
                return Err(StatusCode::BadNotConnected);
            }
        };

        Request::new(request, send, timeout).send().await
    }

    async fn run_inner(&self) -> Result<(), StatusCode> {
        loop {
            // Try to establish a secure channel
            let (transport_fut, send) = self.connect().await?;

            {
                let _ = self.state_watch_tx.send(State::Connected(send));
            }

            let status = transport_fut.await;

            info!("Transport exited with code {status}");

            if status.is_good() || self.token.is_cancelled() {
                break Ok(());
            }

            let _ = self.state_watch_tx.send(State::Connecting);
        }
    }

    async fn connect(
        &self,
    ) -> Result<(Pin<Box<impl Future<Output = StatusCode>>>, RequestSend), StatusCode> {
        loop {
            let mut backoff = self.session_retry_policy.new_backoff();
            match self.connect_no_retry().await {
                Ok(res) => return Ok(res),
                Err(s) => {
                    let Some(delay) = backoff.next() else {
                        return Err(s);
                    };

                    select! {
                        _ = tokio::time::sleep(delay) => {}
                        _ = self.token.cancelled() => {
                            break Err(s);
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn make_request_header(&self, timeout: Duration) -> RequestHeader {
        self.state.make_request_header(timeout)
    }

    pub(crate) fn client_nonce(&self) -> ByteString {
        let secure_channel = trace_read_lock!(self.secure_channel);
        secure_channel.local_nonce_as_byte_string()
    }

    pub(crate) fn update_from_created_session(
        &self,
        nonce: &ByteString,
        certificate: &ByteString,
    ) -> Result<(), StatusCode> {
        let mut secure_channel = trace_write_lock!(self.secure_channel);
        secure_channel.set_remote_nonce_from_byte_string(nonce)?;
        secure_channel.set_remote_cert_from_byte_string(certificate)?;
        Ok(())
    }

    pub(crate) fn security_policy(&self) -> SecurityPolicy {
        let secure_channel = trace_read_lock!(self.secure_channel);
        secure_channel.security_policy()
    }

    async fn connect_no_retry(
        &self,
    ) -> Result<(Pin<Box<impl Future<Output = StatusCode>>>, RequestSend), StatusCode> {
        let (transport, send) = self.create_transport().await?;

        let request = self.state.begin_issue_or_renew_secure_channel(
            SecurityTokenRequestType::Issue,
            Duration::from_secs(30),
            send.clone(),
        )?;

        let fut = if self.run_transport_in_parallel {
            futures::future::Either::Left(transport.run())
        } else {
            futures::future::Either::Right(tokio::task::spawn(transport.run()).map(|r| match r {
                Ok(s) => s,
                Err(_) => StatusCode::BadInternalError,
            }))
        };

        // There is an overhead associated with this, but avoiding it is so hairy that
        // it may not be worth it. If the task is spawned, the polling overhead doesn't matter.
        let mut fut = Box::pin(fut);

        // Temporarily poll the transport task while we're waiting for a response.
        let resp = tokio::select! {
            r = request.send() => r?,
            status = &mut fut => {
                return Err(status);
            }
        };

        self.state.end_issue_or_renew_secure_channel(resp)?;

        Ok((fut, send))
    }

    async fn create_transport(
        &self,
    ) -> Result<(TcpTransport, tokio::sync::mpsc::Sender<OutgoingMessage>), StatusCode> {
        let endpoint_url = self.session_info.endpoint.endpoint_url.clone();
        info!("Connect");
        let security_policy =
            SecurityPolicy::from_str(self.session_info.endpoint.security_policy_uri.as_ref())
                .unwrap();

        if security_policy == SecurityPolicy::Unknown {
            error!(
                "connect, security policy \"{}\" is unknown",
                self.session_info.endpoint.security_policy_uri.as_ref()
            );
            return Err(StatusCode::BadSecurityPolicyRejected);
        } else {
            let (cert, key) = {
                let certificate_store = trace_write_lock!(self.certificate_store);
                certificate_store.read_own_cert_and_pkey_optional()
            };

            {
                let mut secure_channel = trace_write_lock!(self.secure_channel);
                secure_channel.set_private_key(key);
                secure_channel.set_cert(cert);
                secure_channel.set_security_policy(security_policy);
                secure_channel.set_security_mode(self.session_info.endpoint.security_mode);
                let _ = secure_channel.set_remote_cert_from_byte_string(
                    &self.session_info.endpoint.server_certificate,
                );
                info!("Security policy = {:?}", security_policy);
                info!(
                    "Security mode = {:?}",
                    self.session_info.endpoint.security_mode
                );
            }

            let (send, recv) = tokio::sync::mpsc::channel(self.transport_config.max_inflight);
            let transport = TcpTransport::connect(
                self.secure_channel.clone(),
                recv,
                self.transport_config.clone(),
                endpoint_url.as_ref(),
            )
            .await?;

            Ok((transport, send))
        }
    }

    /// Close the secure channel, optionally wait for the channel to close.
    pub async fn close_channel(&self, wait: bool) {
        let request = {
            let msg = CloseSecureChannelRequest {
                request_header: self.state.make_request_header(Duration::from_secs(60)),
            };
            match &*self.state_watch_rx.borrow() {
                State::Connected(sender) => {
                    Some(Request::new(msg, sender.clone(), Duration::from_secs(60)))
                }
                _ => None,
            }
        };

        // Instruct the channel to not attempt to reopen.
        self.token.cancel();

        if let Some(request) = request {
            if let Err(e) = request.send_no_response().await {
                error!("Failed to send disconnect message, queue full: {e}");
                return;
            }
        }

        if wait {
            self.wait_for_state(false).await;
        }
    }

    pub(crate) fn get_state_change_rx(&self) -> tokio::sync::watch::Receiver<State> {
        self.state_watch_rx.clone()
    }
}
