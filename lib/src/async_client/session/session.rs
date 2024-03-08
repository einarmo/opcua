use std::{sync::Arc, time::Duration};

use arc_swap::ArcSwap;
use crypto::user_identity::make_user_name_identity_token;
use futures::future::BoxFuture;
use tokio::{pin, select};

use crate::{
    async_client::{
        retry::{ExponentialBackoff, SessionRetryPolicy},
        transport::{SecureChannelEventLoop, TransportPollResult},
        AsyncSecureChannel,
    },
    client::{
        prelude::{
            encoding::DecodingOptions, hostname_from_url, ActivateSessionRequest,
            AnonymousIdentityToken, ApplicationDescription, ByteString, CertificateStore,
            CreateSessionRequest, DataValue, ExtensionObject, IdentityToken, NodeId, ObjectId,
            ReadRequest, ReadValueId, SecureChannel, SecurityPolicy, SignatureData, StatusCode,
            SupportedMessage, TimestampsToReturn, UAString, UserNameIdentityToken, UserTokenPolicy,
            UserTokenType, X509IdentityToken,
        },
        process_service_result, process_unexpected_response,
    },
    crypto,
    sync::RwLock,
};

use super::SessionInfo;

#[derive(Clone, Copy)]
pub enum SessionState {
    Disconnected,
    Connected,
    Connecting,
}

#[derive(Debug)]
pub enum SessionPollResult {
    Transport(TransportPollResult),
    ConnectionLost(StatusCode),
    ReconnectFailed,
    Reconnected,
    BeginReconnect,
}

struct AsyncSessionState {
    session_id: NodeId,
}

impl AsyncSessionState {
    pub fn reset(&mut self) {
        self.session_id = NodeId::null();
    }
}

struct AsyncSessionInner {
    channel: AsyncSecureChannel,
    state_watch_rx: tokio::sync::watch::Receiver<SessionState>,
    state_watch_tx: tokio::sync::watch::Sender<SessionState>,
    certificate_store: Arc<RwLock<CertificateStore>>,
    state: RwLock<AsyncSessionState>,
    auth_token: Arc<ArcSwap<NodeId>>,
    session_info: SessionInfo,
    session_name: UAString,
    application_description: ApplicationDescription,
}

#[derive(Clone)]
pub struct AsyncSession {
    inner: Arc<AsyncSessionInner>,
}

enum SessionEventLoopState {
    Connected(SecureChannelEventLoop),
    Disconnected,
    Connecting(BoxFuture<'static, Result<SecureChannelEventLoop, StatusCode>>),
}

#[must_use = "The session event loop must be started for the session to work"]
pub struct SessionEventLoop {
    inner: Arc<AsyncSessionInner>,
    state: SessionEventLoopState,
    retry: SessionRetryPolicy,
}

impl SessionEventLoop {
    /// Poll the session.
    pub async fn poll(&mut self) -> Result<SessionPollResult, StatusCode> {
        match &mut self.state {
            SessionEventLoopState::Disconnected => {
                let connector = SessionConnector {
                    inner: self.inner.clone(),
                };

                let fut = Box::pin(connector.run(self.retry.new_backoff()));
                self.state = SessionEventLoopState::Connecting(fut);
                let _ = self.inner.state_watch_tx.send(SessionState::Connecting);

                Ok(SessionPollResult::BeginReconnect)
            }
            SessionEventLoopState::Connected(c) => {
                let r = c.poll().await;
                if let TransportPollResult::Closed(code) = r {
                    log::warn!("Transport disconnected: {code}");

                    self.state = SessionEventLoopState::Disconnected;

                    let _ = self.inner.state_watch_tx.send(SessionState::Disconnected);
                    Ok(SessionPollResult::ConnectionLost(code))
                } else {
                    Ok(SessionPollResult::Transport(r))
                }
            }
            SessionEventLoopState::Connecting(fut) => {
                let c = fut.await?;

                self.state = SessionEventLoopState::Connected(c);
                let _ = self.inner.state_watch_tx.send(SessionState::Connected);

                Ok(SessionPollResult::Reconnected)
            }
        }
    }
}

struct SessionConnector {
    inner: Arc<AsyncSessionInner>,
}

impl SessionConnector {
    pub async fn run(
        self,
        mut backoff: ExponentialBackoff,
    ) -> Result<SecureChannelEventLoop, StatusCode> {
        loop {
            match self.connect_and_activate().await {
                Ok(c) => return Ok(c),
                Err(e) => {
                    log::warn!("Failed to connect to server: {e}");
                    match backoff.next() {
                        Some(b) => tokio::time::sleep(b).await,
                        None => return Err(e),
                    }
                }
            }
        }
    }

    async fn connect_and_activate(&self) -> Result<SecureChannelEventLoop, StatusCode> {
        let mut event_loop = self.inner.channel.connect_no_retry().await?;

        let activate_fut = self.ensure_and_activate_session();
        pin!(activate_fut);

        let res = loop {
            select! {
                r = event_loop.poll() => {
                    if let TransportPollResult::Closed(c) = r {
                        return Err(c);
                    }
                },
                r = &mut activate_fut => break r,
            }
        };

        if let Err(e) = res {
            self.inner.channel.close_channel().await;

            loop {
                if matches!(event_loop.poll().await, TransportPollResult::Closed(_)) {
                    break;
                }
            }

            return Err(e);
        }
        drop(activate_fut);

        Ok(event_loop)
    }

    async fn ensure_and_activate_session(&self) -> Result<(), StatusCode> {
        let should_create_session = {
            let state = trace_read_lock!(self.inner.state);
            state.session_id.is_null()
        };

        if should_create_session {
            self.create_session().await?;
        }

        match self.activate_session().await {
            Err(status_code) if !should_create_session => {
                info!(
                    "Session activation failed on reconnect, error = {}, so creating a new session",
                    status_code
                );
                {
                    let mut session_state = trace_write_lock!(self.inner.state);
                    session_state.reset();
                }
                self.create_session().await?;
                self.activate_session().await?;
            }
            Err(e) => return Err(e),
            Ok(_) => {}
        }

        // TODO: transfer subscriptions
        // self.transfer_subscriptions_from_old_session

        Ok(())
    }

    async fn create_session(&self) -> Result<NodeId, StatusCode> {
        let endpoint_url = self.inner.session_info.endpoint.endpoint_url.clone();

        let client_nonce = self.inner.channel.client_nonce();
        let server_uri = UAString::null();
        let session_name = self.inner.session_name.clone();

        let (client_certificate, _) = {
            let certificate_store = trace_write_lock!(self.inner.certificate_store);
            certificate_store.read_own_cert_and_pkey_optional()
        };

        let client_certificate = if let Some(ref client_certificate) = client_certificate {
            client_certificate.as_byte_string()
        } else {
            ByteString::null()
        };

        let request = CreateSessionRequest {
            request_header: self
                .inner
                .channel
                .make_request_header(Duration::from_secs(30)),
            client_description: self.inner.application_description.clone(),
            server_uri,
            endpoint_url,
            session_name,
            client_nonce,
            client_certificate,
            requested_session_timeout: f64::MAX,
            max_response_message_size: 0,
        };

        let response = self
            .inner
            .channel
            .send(request, Duration::from_secs(30))
            .await?;

        if let SupportedMessage::CreateSessionResponse(response) = response {
            process_service_result(&response.response_header)?;

            let session_id = {
                let mut session_state = trace_write_lock!(self.inner.state);
                session_state.session_id = response.session_id.clone();
                session_state.session_id.clone()
            };
            self.inner
                .auth_token
                .store(Arc::new(response.authentication_token));

            self.inner.channel.update_from_created_session(
                &response.server_nonce,
                &response.server_certificate,
            )?;

            let security_policy = self.inner.channel.security_policy();

            if security_policy != SecurityPolicy::None {
                if let Ok(server_certificate) =
                    crypto::X509::from_byte_string(&response.server_certificate)
                {
                    // Validate server certificate against hostname and application_uri
                    let hostname =
                        hostname_from_url(self.inner.session_info.endpoint.endpoint_url.as_ref())
                            .map_err(|_| StatusCode::BadUnexpectedError)?;
                    let application_uri = self
                        .inner
                        .session_info
                        .endpoint
                        .server
                        .application_uri
                        .as_ref();

                    let certificate_store = trace_write_lock!(self.inner.certificate_store);
                    let result = certificate_store.validate_or_reject_application_instance_cert(
                        &server_certificate,
                        security_policy,
                        Some(&hostname),
                        Some(application_uri),
                    );
                    if result.is_bad() {
                        return Err(result);
                    }
                } else {
                    return Err(StatusCode::BadCertificateInvalid);
                }
            }

            Ok(session_id)
        } else {
            Err(process_unexpected_response(response))
        }
    }

    async fn activate_session(&self) -> Result<(), StatusCode> {
        let secure_channel = trace_read_lock!(self.inner.channel.secure_channel);

        let (user_identity_token, user_token_signature) =
            self.user_identity_token(&secure_channel)?;

        let server_cert = secure_channel.remote_cert();
        let server_nonce = secure_channel.remote_nonce_as_byte_string();

        drop(secure_channel);

        let locale_ids = if self.inner.session_info.preferred_locales.is_empty() {
            None
        } else {
            let locale_ids = self
                .inner
                .session_info
                .preferred_locales
                .iter()
                .map(UAString::from)
                .collect();
            Some(locale_ids)
        };

        let security_policy = self.inner.channel.security_policy();
        let client_signature = match security_policy {
            SecurityPolicy::None => SignatureData::null(),
            _ => {
                let (_, client_pkey) = {
                    let certificate_store = trace_write_lock!(self.inner.certificate_store);
                    certificate_store.read_own_cert_and_pkey_optional()
                };

                // Create a signature data
                if client_pkey.is_none() {
                    error!("Cannot create client signature - no pkey!");
                    return Err(StatusCode::BadUnexpectedError);
                } else if server_cert.is_none() {
                    error!("Cannot sign server certificate because server cert is null");
                    return Err(StatusCode::BadUnexpectedError);
                } else if server_nonce.is_empty() {
                    error!("Cannot sign server certificate because server nonce is empty");
                    return Err(StatusCode::BadUnexpectedError);
                }

                let server_cert = server_cert.unwrap().as_byte_string();
                let signing_key = client_pkey.as_ref().unwrap();
                crypto::create_signature_data(
                    signing_key,
                    security_policy,
                    &server_cert,
                    &server_nonce,
                )?
            }
        };

        let request = ActivateSessionRequest {
            request_header: self
                .inner
                .channel
                .make_request_header(Duration::from_secs(30)),
            client_signature,
            client_software_certificates: None,
            locale_ids,
            user_identity_token,
            user_token_signature,
        };

        let response = self
            .inner
            .channel
            .send(request, Duration::from_secs(30))
            .await?;

        if let SupportedMessage::ActivateSessionResponse(response) = response {
            // trace!("ActivateSessionResponse = {:#?}", response);
            process_service_result(&response.response_header)?;
            Ok(())
        } else {
            Err(process_unexpected_response(response))
        }
    }

    fn user_identity_token(
        &self,
        channel: &SecureChannel,
    ) -> Result<(ExtensionObject, SignatureData), StatusCode> {
        let server_cert = &channel.remote_cert();
        let server_nonce = &channel.remote_nonce();

        let user_identity_token = &self.inner.session_info.user_identity_token;
        let user_token_type = match user_identity_token {
            IdentityToken::Anonymous => UserTokenType::Anonymous,
            IdentityToken::UserName(_, _) => UserTokenType::UserName,
            IdentityToken::X509(_, _) => UserTokenType::Certificate,
        };

        let endpoint = &self.inner.session_info.endpoint;
        let policy = endpoint.find_policy(user_token_type);

        match policy {
            None => {
                error!(
                    "Cannot find user token type {:?} for this endpoint, cannot connect",
                    user_token_type
                );
                Err(StatusCode::BadSecurityPolicyRejected)
            }
            Some(policy) => {
                let security_policy = if policy.security_policy_uri.is_null() {
                    // Assume None
                    SecurityPolicy::None
                } else {
                    SecurityPolicy::from_uri(policy.security_policy_uri.as_ref())
                };

                if security_policy == SecurityPolicy::Unknown {
                    error!("Unknown security policy {}", policy.security_policy_uri);
                    return Err(StatusCode::BadSecurityPolicyRejected);
                }

                match &user_identity_token {
                    IdentityToken::Anonymous => {
                        let identity_token = AnonymousIdentityToken {
                            policy_id: policy.policy_id.clone(),
                        };
                        let identity_token = ExtensionObject::from_encodable(
                            ObjectId::UserNameIdentityToken_Encoding_DefaultBinary,
                            &identity_token,
                        );
                        Ok((identity_token, SignatureData::null()))
                    }
                    IdentityToken::UserName(user, pass) => {
                        let identity_token =
                            self.make_user_name_identity_token(channel, policy, user, pass)?;
                        let identity_token = ExtensionObject::from_encodable(
                            ObjectId::UserNameIdentityToken_Encoding_DefaultBinary,
                            &identity_token,
                        );
                        Ok((identity_token, SignatureData::null()))
                    }
                    IdentityToken::X509(cert_path, private_key_path) => {
                        let Some(server_cert) = &server_cert else {
                            error!("Cannot create an X509IdentityToken because the remote server has no cert with which to create a signature");
                            return Err(StatusCode::BadCertificateInvalid);
                        };
                        let certificate_data =
                            CertificateStore::read_cert(cert_path).map_err(|e| {
                                error!(
                                    "Certificate cannot be loaded from path {}, error = {}",
                                    cert_path.to_str().unwrap(),
                                    e
                                );
                                StatusCode::BadSecurityPolicyRejected
                            })?;
                        let private_key =
                            CertificateStore::read_pkey(private_key_path).map_err(|e| {
                                error!(
                                    "Private key cannot be loaded from path {}, error = {}",
                                    private_key_path.to_str().unwrap(),
                                    e
                                );
                                StatusCode::BadSecurityPolicyRejected
                            })?;
                        let user_token_signature = crypto::create_signature_data(
                            &private_key,
                            security_policy,
                            &server_cert.as_byte_string(),
                            &ByteString::from(server_nonce),
                        )?;

                        // Create identity token
                        let identity_token = X509IdentityToken {
                            policy_id: policy.policy_id.clone(),
                            certificate_data: certificate_data.as_byte_string(),
                        };
                        let identity_token = ExtensionObject::from_encodable(
                            ObjectId::X509IdentityToken_Encoding_DefaultBinary,
                            &identity_token,
                        );

                        Ok((identity_token, user_token_signature))
                    }
                }
            }
        }
    }

    fn make_user_name_identity_token(
        &self,
        secure_channel: &SecureChannel,
        user_token_policy: &UserTokenPolicy,
        user: &str,
        pass: &str,
    ) -> Result<UserNameIdentityToken, StatusCode> {
        let channel_security_policy = secure_channel.security_policy();
        let nonce = secure_channel.remote_nonce();
        let cert = secure_channel.remote_cert();
        make_user_name_identity_token(
            channel_security_policy,
            user_token_policy,
            nonce,
            &cert,
            user,
            pass,
        )
    }
}

impl AsyncSession {
    pub(crate) fn new(
        certificate_store: Arc<RwLock<CertificateStore>>,
        session_info: SessionInfo,
        session_name: UAString,
        application_description: ApplicationDescription,
        session_retry_policy: SessionRetryPolicy,
        decoding_options: DecodingOptions,
        ignore_clock_skew: bool,
    ) -> (Self, SessionEventLoop) {
        let auth_token: Arc<ArcSwap<NodeId>> = Default::default();
        let (state_watch_tx, state_watch_rx) =
            tokio::sync::watch::channel(SessionState::Disconnected);

        let inner = Arc::new(AsyncSessionInner {
            channel: AsyncSecureChannel::new(
                certificate_store.clone(),
                session_info.clone(),
                session_retry_policy.clone(),
                decoding_options,
                ignore_clock_skew,
                auth_token.clone(),
            ),
            state_watch_rx,
            state_watch_tx,
            state: RwLock::new(AsyncSessionState {
                session_id: NodeId::null(),
            }),
            session_info,
            auth_token,
            session_name,
            application_description,
            certificate_store,
        });

        (
            Self {
                inner: inner.clone(),
            },
            SessionEventLoop {
                inner,
                state: SessionEventLoopState::Disconnected,
                retry: session_retry_policy,
            },
        )
    }

    pub async fn read(
        &self,
        nodes_to_read: &[ReadValueId],
        timestamps_to_return: TimestampsToReturn,
        max_age: f64,
    ) -> Result<Vec<DataValue>, StatusCode> {
        if nodes_to_read.is_empty() {
            return Err(StatusCode::BadNothingToDo);
        }

        let request = ReadRequest {
            request_header: self
                .inner
                .channel
                .make_request_header(Duration::from_secs(30)),
            max_age,
            timestamps_to_return,
            nodes_to_read: Some(nodes_to_read.to_vec()),
        };

        let response = self
            .inner
            .channel
            .send(request, Duration::from_secs(30))
            .await?;

        if let SupportedMessage::ReadResponse(response) = response {
            process_service_result(&response.response_header)?;
            Ok(response.results.unwrap_or_default())
        } else {
            Err(process_unexpected_response(response))
        }
    }

    async fn wait_for_state(&self, connected: bool) -> bool {
        let mut rx = self.inner.state_watch_rx.clone();

        loop {
            if !rx.changed().await.is_ok() {
                return false;
            };
            {
                let state = rx.borrow();
                if connected && matches!(*state, SessionState::Connected)
                    || !connected && matches!(*state, SessionState::Disconnected)
                {
                    return true;
                }
            }
        }
    }

    pub async fn wait_for_connection(&self) -> bool {
        self.wait_for_state(true).await
    }
}
