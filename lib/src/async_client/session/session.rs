use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use arc_swap::ArcSwap;

use crate::{
    async_client::{retry::SessionRetryPolicy, AsyncSecureChannel},
    client::prelude::{
        encoding::DecodingOptions, ApplicationDescription, CertificateStore, ClientConfig, NodeId,
        RequestHeader, StatusCode, SupportedMessage, UAString,
    },
    core::handle::AtomicHandle,
    sync::{Mutex, RwLock},
};

use super::{services::subscriptions::state::SubscriptionState, SessionEventLoop, SessionInfo};

#[derive(Clone, Copy)]
pub enum SessionState {
    Disconnected,
    Connected,
    Connecting,
}

lazy_static! {
    static ref NEXT_SESSION_ID: AtomicU32 = AtomicU32::new(1);
}

pub struct AsyncSession {
    pub(super) channel: AsyncSecureChannel,
    pub(super) state_watch_rx: tokio::sync::watch::Receiver<SessionState>,
    pub(super) state_watch_tx: tokio::sync::watch::Sender<SessionState>,
    pub(super) certificate_store: Arc<RwLock<CertificateStore>>,
    pub(super) session_id: Arc<ArcSwap<NodeId>>,
    pub(super) auth_token: Arc<ArcSwap<NodeId>>,
    pub(super) internal_session_id: AtomicU32,
    pub(super) session_info: SessionInfo,
    pub(super) session_name: UAString,
    pub(super) application_description: ApplicationDescription,
    pub(super) request_timeout: Duration,
    pub(super) publish_timeout: Duration,
    pub(super) recreate_monitored_items_chunk: usize,
    pub(super) session_timeout: f64,
    pub(super) max_inflight_publish: usize,
    pub subscription_state: Mutex<SubscriptionState>,
    pub(super) monitored_item_handle: AtomicHandle,
    pub(super) trigger_publish_tx: tokio::sync::watch::Sender<Instant>,
}

impl AsyncSession {
    pub(crate) fn new(
        certificate_store: Arc<RwLock<CertificateStore>>,
        session_info: SessionInfo,
        session_name: UAString,
        application_description: ApplicationDescription,
        session_retry_policy: SessionRetryPolicy,
        decoding_options: DecodingOptions,
        config: &ClientConfig,
    ) -> (Arc<Self>, SessionEventLoop) {
        let auth_token: Arc<ArcSwap<NodeId>> = Default::default();
        let (state_watch_tx, state_watch_rx) =
            tokio::sync::watch::channel(SessionState::Disconnected);
        let (trigger_publish_tx, trigger_publish_rx) = tokio::sync::watch::channel(Instant::now());

        let session = Arc::new(AsyncSession {
            channel: AsyncSecureChannel::new(
                certificate_store.clone(),
                session_info.clone(),
                session_retry_policy.clone(),
                decoding_options,
                config.performance.ignore_clock_skew,
                auth_token.clone(),
            ),
            internal_session_id: AtomicU32::new(NEXT_SESSION_ID.fetch_add(1, Ordering::Relaxed)),
            state_watch_rx,
            state_watch_tx,
            session_id: Default::default(),
            session_info,
            auth_token,
            session_name,
            application_description,
            certificate_store,
            request_timeout: config.request_timeout,
            session_timeout: config.session_timeout as f64,
            publish_timeout: config.publish_timeout,
            max_inflight_publish: config.max_inflight_publish,
            recreate_monitored_items_chunk: config.performance.recreate_monitored_items_chunk,
            subscription_state: Mutex::new(SubscriptionState::new(config.min_publish_interval)),
            monitored_item_handle: AtomicHandle::new(1000),
            trigger_publish_tx,
        });

        (
            session.clone(),
            SessionEventLoop::new(session, session_retry_policy, trigger_publish_rx),
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

    pub(crate) fn reset(&self) {
        self.session_id.store(Arc::new(NodeId::null()));
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
