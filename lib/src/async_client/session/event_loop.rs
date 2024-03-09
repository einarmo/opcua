use std::sync::Arc;

use futures::{Stream, TryStream};

use crate::{
    async_client::{
        retry::SessionRetryPolicy,
        transport::{SecureChannelEventLoop, TransportPollResult},
    },
    client::prelude::StatusCode,
};

use super::{
    connect::SessionConnector,
    session::{SessionPollResult, SessionState},
    AsyncSession,
};

enum SessionEventLoopState {
    Connected(SecureChannelEventLoop),
    Connecting(SessionConnector),
    Disconnected,
}
#[must_use = "The session event loop must be started for the session to work"]
pub struct SessionEventLoop {
    inner: Arc<AsyncSession>,
    retry: SessionRetryPolicy,
}

impl SessionEventLoop {
    pub fn new(inner: Arc<AsyncSession>, retry: SessionRetryPolicy) -> Self {
        Self { inner, retry }
    }

    pub fn run(self) -> impl Stream<Item = Result<SessionPollResult, StatusCode>> {
        futures::stream::try_unfold(
            (self, SessionEventLoopState::Disconnected),
            |(slf, state)| async move {
                let (res, state) = match state {
                    SessionEventLoopState::Connected(mut c) => {
                        let r = c.poll().await;
                        if let TransportPollResult::Closed(code) = r {
                            log::warn!("Transport disconnected: {code}");

                            if code.is_good() {
                                return Ok(None);
                            }

                            let _ = slf.inner.state_watch_tx.send(SessionState::Disconnected);
                            Ok((
                                SessionPollResult::ConnectionLost(code),
                                SessionEventLoopState::Disconnected,
                            ))
                        } else {
                            Ok((
                                SessionPollResult::Transport(r),
                                SessionEventLoopState::Connected(c),
                            ))
                        }
                    }
                    SessionEventLoopState::Disconnected => {
                        let connector = SessionConnector::new(slf.inner.clone());

                        let _ = slf.inner.state_watch_tx.send(SessionState::Connecting);

                        Ok((
                            SessionPollResult::BeginReconnect,
                            SessionEventLoopState::Connecting(connector),
                        ))
                    }
                    SessionEventLoopState::Connecting(c) => {
                        let c = c.run(slf.retry.new_backoff()).await?;

                        let _ = slf.inner.state_watch_tx.send(SessionState::Connected);

                        Ok((
                            SessionPollResult::Reconnected,
                            SessionEventLoopState::Connected(c),
                        ))
                    }
                }?;

                Ok(Some((res, (slf, state))))
            },
        )
    }
}

enum SessionTickEvent {
    KeepAlive,
}

struct SessionIntervals {
    keep_alive: tokio::time::Interval, // Subscription time also goes here
}

struct SessionActivityLoop {
    inner: Arc<AsyncSession>,
    tick_gen: SessionIntervals,
}

impl SessionActivityLoop {
    /* pub async fn run(self) -> impl TryStream<Ok = SessionPollResult, Error = StatusCode> {
        futures::stream::try_unfold(self, |mut slf| {})
    } */
}
