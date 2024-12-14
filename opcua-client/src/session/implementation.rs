use std::{
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use arc_swap::ArcSwap;

use crate::{
    retry::SessionRetryPolicy,
    session::session_warn,
    transport::{tcp::TransportConfiguration, Connector},
    AsyncSecureChannel, ClientConfig,
};
use opcua_core::{
    handle::AtomicHandle,
    sync::{Mutex, RwLock},
};
use opcua_crypto::CertificateStore;
use opcua_types::{
    ApplicationDescription, ContextOwned, DecodingOptions, IntegerId, NamespaceMap, NodeId,
    RequestHeader, StatusCode, TypeLoader, UAString,
};

use super::{
    services::subscriptions::{state::SubscriptionState, PublishLimits},
    SessionEventLoop, SessionInfo,
};

#[derive(Clone, Copy)]
pub enum SessionState {
    Disconnected,
    Connected,
    Connecting,
}

lazy_static::lazy_static! {
    static ref NEXT_SESSION_ID: AtomicU32 = AtomicU32::new(1);
}

/// An OPC-UA session. This session provides methods for all supported services that require an open session.
///
/// Note that not all servers may support all service requests and calling an unsupported API
/// may cause the connection to be dropped. Your client is expected to know the capabilities of
/// the server it is calling to avoid this.
///
pub struct Session {
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
    /// Reference to the subscription cache for the client.
    pub subscription_state: Mutex<SubscriptionState>,
    pub(super) publish_limits_watch_rx: tokio::sync::watch::Receiver<PublishLimits>,
    pub(super) publish_limits_watch_tx: tokio::sync::watch::Sender<PublishLimits>,
    pub(super) monitored_item_handle: AtomicHandle,
    pub(super) trigger_publish_tx: tokio::sync::watch::Sender<Instant>,
    decoding_options: DecodingOptions,
    pub(super) should_reconnect: AtomicBool,
    pub(super) auto_recreate_subscriptions: bool,
    pub(super) encoding_context: Arc<RwLock<ContextOwned>>,
}

impl Session {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        certificate_store: Arc<RwLock<CertificateStore>>,
        session_info: SessionInfo,
        session_name: UAString,
        application_description: ApplicationDescription,
        session_retry_policy: SessionRetryPolicy,
        decoding_options: DecodingOptions,
        config: &ClientConfig,
        session_id: Option<NodeId>,
        connector: Box<dyn Connector>,
        extra_type_loaders: Vec<Arc<dyn TypeLoader>>,
    ) -> (Arc<Self>, SessionEventLoop) {
        let auth_token: Arc<ArcSwap<NodeId>> = Arc::default();
        let (publish_limits_watch_tx, publish_limits_watch_rx) =
            tokio::sync::watch::channel(PublishLimits::new());
        let (state_watch_tx, state_watch_rx) =
            tokio::sync::watch::channel(SessionState::Disconnected);
        let (trigger_publish_tx, trigger_publish_rx) = tokio::sync::watch::channel(Instant::now());

        let mut encoding_context =
            ContextOwned::new_default(NamespaceMap::new(), decoding_options.clone());

        for loader in extra_type_loaders {
            encoding_context.loaders_mut().add(loader);
        }

        let encoding_context = Arc::new(RwLock::new(encoding_context));

        let session = Arc::new(Session {
            channel: AsyncSecureChannel::new(
                certificate_store.clone(),
                session_info.clone(),
                session_retry_policy.clone(),
                config.performance.ignore_clock_skew,
                auth_token.clone(),
                TransportConfiguration {
                    max_pending_incoming: 5,
                    send_buffer_size: config.decoding_options.max_chunk_size,
                    recv_buffer_size: config.decoding_options.max_incoming_chunk_size,
                    max_message_size: config.decoding_options.max_message_size,
                    max_chunk_count: config.decoding_options.max_chunk_count,
                },
                connector,
                config.channel_lifetime,
                encoding_context.clone(),
            ),
            internal_session_id: AtomicU32::new(NEXT_SESSION_ID.fetch_add(1, Ordering::Relaxed)),
            state_watch_rx,
            state_watch_tx,
            session_id: Arc::new(ArcSwap::new(Arc::new(session_id.unwrap_or_default()))),
            session_info,
            auth_token,
            session_name,
            application_description,
            certificate_store,
            request_timeout: config.request_timeout,
            session_timeout: config.session_timeout as f64,
            publish_timeout: config.publish_timeout,
            recreate_monitored_items_chunk: config.performance.recreate_monitored_items_chunk,
            subscription_state: Mutex::new(SubscriptionState::new(
                config.min_publish_interval,
                publish_limits_watch_tx.clone(),
            )),
            monitored_item_handle: AtomicHandle::new(1000),
            publish_limits_watch_rx,
            publish_limits_watch_tx,
            trigger_publish_tx,
            decoding_options,
            should_reconnect: AtomicBool::new(true),
            auto_recreate_subscriptions: config.auto_recreate_subscriptions,
            encoding_context,
        });

        (
            session.clone(),
            SessionEventLoop::new(
                session,
                session_retry_policy,
                trigger_publish_rx,
                config.keep_alive_interval,
                config.max_failed_keep_alive_count,
            ),
        )
    }

    /// Create a request header with the default timeout.
    pub(super) fn make_request_header(&self) -> RequestHeader {
        self.channel.make_request_header(self.request_timeout)
    }

    /// Reset the session after a hard disconnect, clearing the session ID and incrementing the internal
    /// session counter.
    pub(crate) fn reset(&self) {
        self.session_id.store(Arc::new(NodeId::null()));
        self.internal_session_id.store(
            NEXT_SESSION_ID.fetch_add(1, Ordering::Relaxed),
            Ordering::Relaxed,
        );
    }

    /// Wait for the session to be in either a connected or disconnected state.
    async fn wait_for_state(&self, connected: bool) -> bool {
        let mut rx = self.state_watch_rx.clone();

        let res = rx
            .wait_for(|s| {
                connected && matches!(*s, SessionState::Connected)
                    || !connected && matches!(*s, SessionState::Disconnected)
            })
            .await
            .is_ok();

        // Compiler limitation
        #[allow(clippy::let_and_return)]
        res
    }

    /// The internal ID of the session, used to keep track of multiple sessions in the same program.
    pub fn session_id(&self) -> u32 {
        self.internal_session_id.load(Ordering::Relaxed)
    }

    /// Get the current session ID. This is different from `session_id`, which is the client-side ID
    /// to keep track of multiple sessions. This is the session ID the server uses to identify this session.
    pub fn server_session_id(&self) -> NodeId {
        (**(*self.session_id).load()).clone()
    }

    /// Convenience method to wait for a connection to the server.
    ///
    /// You should also monitor the session event loop. If it ends, this method will never return.
    pub async fn wait_for_connection(&self) -> bool {
        self.wait_for_state(true).await
    }

    /// Disable automatic reconnects.
    /// This will make the event loop quit the next time
    /// it disconnects for whatever reason.
    pub fn disable_reconnects(&self) {
        self.should_reconnect.store(false, Ordering::Relaxed);
    }

    /// Enable automatic reconnects.
    /// Automatically reconnecting is enabled by default.
    pub fn enable_reconnects(&self) {
        self.should_reconnect.store(true, Ordering::Relaxed);
    }

    /// Inner method for disconnect. [`Session::disconnect`] and [`Session::disconnect_without_delete_subscriptions`]
    /// are shortands for this with `delete_subscriptions` set to `false` and `true` respectively, and
    /// `disable_reconnect` set to `true`.
    pub async fn disconnect_inner(
        &self,
        delete_subscriptions: bool,
        disable_reconnect: bool,
    ) -> Result<(), StatusCode> {
        if disable_reconnect {
            self.should_reconnect.store(false, Ordering::Relaxed);
        }
        let mut res = Ok(());
        if let Err(e) = self.close_session(delete_subscriptions).await {
            res = Err(e);
            session_warn!(
                self,
                "Failed to close session, channel will be closed anyway: {e}"
            );
        }
        self.channel.close_channel().await;

        self.wait_for_state(false).await;

        res
    }

    /// Disconnect from the server and wait until disconnected.
    /// This will set the `should_reconnect` flag to false on the session, indicating
    /// that it should not attempt to reconnect to the server. You may clear this flag
    /// yourself to
    pub async fn disconnect(&self) -> Result<(), StatusCode> {
        self.disconnect_inner(true, true).await
    }

    /// Disconnect the server without deleting subscriptions, then wait until disconnected.
    pub async fn disconnect_without_delete_subscriptions(&self) -> Result<(), StatusCode> {
        self.disconnect_inner(false, true).await
    }

    /// Get the decoding options used by the session.
    pub fn decoding_options(&self) -> &DecodingOptions {
        &self.decoding_options
    }

    /// Get a reference to the inner secure channel.
    pub fn channel(&self) -> &AsyncSecureChannel {
        &self.channel
    }

    /// Get the next request handle.
    pub fn request_handle(&self) -> IntegerId {
        self.channel.request_handle()
    }

    /// Set the namespace array on the session.
    /// Make sure that this namespace array contains the base namespace,
    /// or the session may behave unexpectedly.
    pub fn set_namespaces(&mut self, namespaces: NamespaceMap) {
        *self.encoding_context.write().namespaces_mut() = namespaces;
    }

    /// Add a type loader to the encoding context.
    /// Note that there is no mechanism to ensure uniqueness,
    /// you should avoid adding the same type loader more than once, it will
    /// work, but there will be a small performance overhead.
    pub fn add_type_loader(&mut self, type_loader: Arc<dyn TypeLoader>) {
        self.encoding_context.write().loaders_mut().add(type_loader);
    }

    /// Get a reference to the encoding
    pub fn context(&self) -> Arc<RwLock<ContextOwned>> {
        self.channel.secure_channel.read().context_arc()
    }
}
