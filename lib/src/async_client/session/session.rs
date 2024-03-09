use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

use arc_swap::ArcSwap;

use crate::{
    async_client::{retry::SessionRetryPolicy, AsyncSecureChannel},
    client::{
        prelude::{
            encoding::DecodingOptions, ApplicationDescription, CertificateStore, DataValue, NodeId,
            ReadRequest, ReadValueId, RequestHeader, StatusCode, SupportedMessage,
            TimestampsToReturn, UAString,
        },
        process_service_result, process_unexpected_response,
    },
    sync::RwLock,
};

use super::{SessionEventLoop, SessionInfo};

#[derive(Clone, Copy)]
pub enum SessionState {
    Disconnected,
    Connected,
    Connecting,
}

pub(super) struct AsyncSessionState {
    pub(super) session_id: NodeId,
}

impl AsyncSessionState {
    pub fn reset(&mut self) {
        self.session_id = NodeId::null();
    }
}

lazy_static! {
    static ref NEXT_SESSION_ID: AtomicU32 = AtomicU32::new(1);
}

pub struct AsyncSession {
    pub(super) channel: AsyncSecureChannel,
    pub(super) state_watch_rx: tokio::sync::watch::Receiver<SessionState>,
    pub(super) state_watch_tx: tokio::sync::watch::Sender<SessionState>,
    pub(super) certificate_store: Arc<RwLock<CertificateStore>>,
    pub(super) state: RwLock<AsyncSessionState>,
    pub(super) auth_token: Arc<ArcSwap<NodeId>>,
    pub(super) internal_session_id: AtomicU32,
    pub(super) session_info: SessionInfo,
    pub(super) session_name: UAString,
    pub(super) application_description: ApplicationDescription,
    pub(super) request_timeout: Duration,
    pub(super) session_timeout: f64,
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
        request_timeout: Duration,
        session_timeout: f64,
    ) -> (Arc<Self>, SessionEventLoop) {
        let auth_token: Arc<ArcSwap<NodeId>> = Default::default();
        let (state_watch_tx, state_watch_rx) =
            tokio::sync::watch::channel(SessionState::Disconnected);

        let session = Arc::new(AsyncSession {
            channel: AsyncSecureChannel::new(
                certificate_store.clone(),
                session_info.clone(),
                session_retry_policy.clone(),
                decoding_options,
                ignore_clock_skew,
                auth_token.clone(),
            ),
            internal_session_id: AtomicU32::new(NEXT_SESSION_ID.fetch_add(1, Ordering::Relaxed)),
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
            request_timeout,
            session_timeout,
        });

        (
            session.clone(),
            SessionEventLoop::new(session, session_retry_policy),
        )
    }

    pub(super) async fn send(
        &self,
        request: impl Into<SupportedMessage>,
    ) -> Result<SupportedMessage, StatusCode> {
        self.channel.send(request, self.request_timeout).await
    }

    pub(super) fn make_request_header(&self) -> RequestHeader {
        self.channel.make_request_header(self.request_timeout)
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
            request_header: self.make_request_header(),
            max_age,
            timestamps_to_return,
            nodes_to_read: Some(nodes_to_read.to_vec()),
        };

        let response = self.send(request).await?;

        if let SupportedMessage::ReadResponse(response) = response {
            process_service_result(&response.response_header)?;
            Ok(response.results.unwrap_or_default())
        } else {
            Err(process_unexpected_response(response))
        }
    }

    pub(crate) fn reset(&self) {
        {
            let mut session_state = trace_write_lock!(self.state);
            session_state.reset();
        }
        self.internal_session_id.store(
            NEXT_SESSION_ID.fetch_add(1, Ordering::Relaxed),
            Ordering::Relaxed,
        );
    }

    async fn wait_for_state(&self, connected: bool) -> bool {
        let mut rx = self.state_watch_rx.clone();

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

    pub fn session_id(&self) -> u32 {
        self.internal_session_id.load(Ordering::Relaxed)
    }

    pub async fn wait_for_connection(&self) -> bool {
        self.wait_for_state(true).await
    }

    pub async fn disconnect(&self) -> Result<(), StatusCode> {
        self.close_session().await?;
        self.channel.close_channel().await;

        self.wait_for_state(false).await;

        Ok(())
    }
}
