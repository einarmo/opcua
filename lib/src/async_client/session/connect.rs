use std::sync::Arc;

use tokio::{pin, select};

use crate::{
    async_client::transport::{SecureChannelEventLoop, TransportPollResult},
    client::prelude::{NodeId, StatusCode},
};

use super::AsyncSession;

pub(super) struct SessionConnector {
    inner: Arc<AsyncSession>,
}

#[derive(Debug, Clone)]
pub enum SessionReconnectMode {
    NewSession(NodeId),
    ReactivatedSession(NodeId),
}

impl SessionConnector {
    pub fn new(session: Arc<AsyncSession>) -> Self {
        Self { inner: session }
    }

    pub async fn try_connect(
        &self,
    ) -> Result<(SecureChannelEventLoop, SessionReconnectMode), StatusCode> {
        self.connect_and_activate().await
    }

    async fn connect_and_activate(
        &self,
    ) -> Result<(SecureChannelEventLoop, SessionReconnectMode), StatusCode> {
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

        let id = match res {
            Ok(id) => id,
            Err(e) => {
                self.inner.channel.close_channel().await;

                loop {
                    if matches!(event_loop.poll().await, TransportPollResult::Closed(_)) {
                        break;
                    }
                }

                return Err(e);
            }
        };

        drop(activate_fut);

        Ok((event_loop, id))
    }

    async fn ensure_and_activate_session(&self) -> Result<SessionReconnectMode, StatusCode> {
        let should_create_session = self.inner.session_id.load().is_null();

        if should_create_session {
            self.inner.create_session().await?;
        }

        let reconnect = match self.inner.activate_session().await {
            Err(status_code) if !should_create_session => {
                info!(
                    "Session activation failed on reconnect, error = {}, creating a new session",
                    status_code
                );
                self.inner.reset();
                let id = self.inner.create_session().await?;
                self.inner.activate_session().await?;
                SessionReconnectMode::NewSession(id)
            }
            Err(e) => return Err(e),
            Ok(_) => {
                let session_id = (**self.inner.session_id.load()).clone();
                if should_create_session {
                    SessionReconnectMode::NewSession(session_id)
                } else {
                    SessionReconnectMode::ReactivatedSession(session_id)
                }
            }
        };

        self.inner.transfer_subscriptions_from_old_session().await;

        Ok(reconnect)
    }
}
