use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures::{stream::BoxStream, Stream, StreamExt, TryStreamExt};

use crate::{
    async_client::{
        retry::{ExponentialBackoff, SessionRetryPolicy},
        session::{session_error, session_warn},
        transport::{SecureChannelEventLoop, TransportPollResult},
    },
    client::prelude::{AttributeId, QualifiedName, ReadValueId, StatusCode, VariableId},
};

use super::{
    connect::{SessionConnector, SessionReconnectMode},
    services::subscriptions::event_loop::{SubscriptionActivity, SubscriptionEventLoop},
    session::SessionState,
    AsyncSession,
};

#[derive(Debug)]
pub enum SessionPollResult {
    Transport(TransportPollResult),
    ConnectionLost(StatusCode),
    ReconnectFailed(StatusCode),
    Reconnected(SessionReconnectMode),
    SessionActivity(SessionActivity),
    Subscription(SubscriptionActivity),
    BeginReconnect,
}

enum SessionEventLoopState {
    Connected(
        SecureChannelEventLoop,
        BoxStream<'static, SessionActivity>,
        BoxStream<'static, SubscriptionActivity>,
    ),
    Connecting(SessionConnector, ExponentialBackoff, Instant),
    Disconnected,
}

#[must_use = "The session event loop must be started for the session to work"]
pub struct SessionEventLoop {
    inner: Arc<AsyncSession>,
    trigger_publish_recv: tokio::sync::watch::Receiver<Instant>,
    retry: SessionRetryPolicy,
}

impl SessionEventLoop {
    pub fn new(
        inner: Arc<AsyncSession>,
        retry: SessionRetryPolicy,
        trigger_publish_recv: tokio::sync::watch::Receiver<Instant>,
    ) -> Self {
        Self {
            inner,
            retry,
            trigger_publish_recv,
        }
    }

    /// Convenience method for running the session event loop until completion,
    /// this method will return once the session is closed manually, or
    /// after it fails to reconnect.
    pub async fn run(self) -> StatusCode {
        let stream = self.enter();
        tokio::pin!(stream);
        loop {
            let r = stream.try_next().await;

            match r {
                Ok(None) => break StatusCode::Good,
                Err(e) => break e,
                _ => (),
            }
        }
    }

    /// Start the event loop, returning a stream that must be polled until it is closed.
    /// The stream will return `None` when the transport is closed manually, or
    /// `Some(Err(StatusCode))` when the stream fails to reconnect after a loss of connection.
    ///
    /// It yields events from normal session operation, which can be used to take specific actions
    /// based on changes to the session state.
    pub fn enter(self) -> impl Stream<Item = Result<SessionPollResult, StatusCode>> {
        futures::stream::try_unfold(
            (self, SessionEventLoopState::Disconnected),
            |(slf, state)| async move {
                let (res, state) = match state {
                    SessionEventLoopState::Connected(mut c, mut activity, mut subscriptions) => {
                        tokio::select! {
                            r = c.poll() => {
                                if let TransportPollResult::Closed(code) = r {
                                    session_warn!(slf.inner, "Transport disconnected: {code}");
                                    let _ = slf.inner.state_watch_tx.send(SessionState::Disconnected);

                                    if code.is_good() {
                                        return Ok(None);
                                    }

                                    Ok((
                                        SessionPollResult::ConnectionLost(code),
                                        SessionEventLoopState::Disconnected,
                                    ))
                                } else {
                                    Ok((
                                        SessionPollResult::Transport(r),
                                        SessionEventLoopState::Connected(c, activity, subscriptions),
                                    ))
                                }
                            }
                            r = activity.next() => {
                                // Should never be null, fail out
                                let Some(r) = r else {
                                    session_error!(slf.inner, "Session activity loop ended unexpectedly");
                                    return Err(StatusCode::BadUnexpectedError);
                                };

                                Ok((
                                    SessionPollResult::SessionActivity(r),
                                    SessionEventLoopState::Connected(c, activity, subscriptions),
                                ))
                            }
                            r = subscriptions.next() => {
                                // Should never be null, fail out
                                let Some(r) = r else {
                                    session_error!(slf.inner, "Subscription event loop ended unexpectedly");
                                    return Err(StatusCode::BadUnexpectedError);
                                };

                                Ok((
                                    SessionPollResult::Subscription(r),
                                    SessionEventLoopState::Connected(c, activity, subscriptions),
                                ))
                            }
                        }
                    }
                    SessionEventLoopState::Disconnected => {
                        let connector = SessionConnector::new(slf.inner.clone());

                        let _ = slf.inner.state_watch_tx.send(SessionState::Connecting);

                        Ok((
                            SessionPollResult::BeginReconnect,
                            SessionEventLoopState::Connecting(
                                connector,
                                slf.retry.new_backoff(),
                                Instant::now(),
                            ),
                        ))
                    }
                    SessionEventLoopState::Connecting(connector, mut backoff, next_try) => {
                        tokio::time::sleep_until(next_try.into()).await;

                        match connector.try_connect().await {
                            Ok((channel, result)) => {
                                let _ = slf.inner.state_watch_tx.send(SessionState::Connected);
                                Ok((
                                    SessionPollResult::Reconnected(result),
                                    SessionEventLoopState::Connected(
                                        channel,
                                        SessionActivityLoop::new(
                                            slf.inner.clone(),
                                            slf.retry.keep_alive_interval(),
                                        )
                                        .run()
                                        .boxed(),
                                        SubscriptionEventLoop::new(
                                            slf.inner.clone(),
                                            slf.trigger_publish_recv.clone(),
                                        )
                                        .run()
                                        .boxed(),
                                    ),
                                ))
                            }
                            Err(e) => match backoff.next() {
                                Some(x) => Ok((
                                    SessionPollResult::ReconnectFailed(e),
                                    SessionEventLoopState::Connecting(
                                        connector,
                                        backoff,
                                        Instant::now() + x,
                                    ),
                                )),
                                None => Err(e),
                            },
                        }
                    }
                }?;

                Ok(Some((res, (slf, state))))
            },
        )
    }
}

#[derive(Debug, Clone)]
pub enum SessionActivity {
    KeepAliveSucceeded,
    KeepAliveFailed(StatusCode),
    Publish,
}

enum SessionTickEvent {
    KeepAlive,
}

struct SessionIntervals {
    keep_alive: tokio::time::Interval, // Subscription time also goes here
}

impl SessionIntervals {
    pub fn new(keep_alive_interval: Duration) -> Self {
        let mut keep_alive = tokio::time::interval(keep_alive_interval);
        keep_alive.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        Self { keep_alive }
    }

    pub async fn next(&mut self) -> SessionTickEvent {
        tokio::select! {
            _ = self.keep_alive.tick() => SessionTickEvent::KeepAlive
        }
    }
}

struct SessionActivityLoop {
    inner: Arc<AsyncSession>,
    tick_gen: SessionIntervals,
}

impl SessionActivityLoop {
    pub fn new(inner: Arc<AsyncSession>, keep_alive_interval: Duration) -> Self {
        Self {
            inner,
            tick_gen: SessionIntervals::new(keep_alive_interval),
        }
    }

    pub fn run(self) -> impl Stream<Item = SessionActivity> {
        futures::stream::unfold(self, |mut slf| async move {
            match slf.tick_gen.next().await {
                SessionTickEvent::KeepAlive => {
                    let res = slf
                        .inner
                        .read(
                            &[ReadValueId {
                                node_id: VariableId::Server_ServerStatus_State.into(),
                                attribute_id: AttributeId::Value as u32,
                                index_range: Default::default(),
                                data_encoding: QualifiedName::null(),
                            }],
                            crate::client::prelude::TimestampsToReturn::Server,
                            1f64,
                        )
                        .await;

                    let value = match res.map(|r| r.into_iter().next()) {
                        Ok(Some(dv)) => dv,
                        // Should not be possible, this would be a bug in the server, assume everything
                        // is terrible.
                        Ok(None) => {
                            return Some((
                                SessionActivity::KeepAliveFailed(StatusCode::BadUnknownResponse),
                                slf,
                            ))
                        }
                        Err(e) => return Some((SessionActivity::KeepAliveFailed(e), slf)),
                    };

                    let Some(status): Option<u8> = value.value.and_then(|v| v.try_into().ok())
                    else {
                        return Some((
                            SessionActivity::KeepAliveFailed(StatusCode::BadUnknownResponse),
                            slf,
                        ));
                    };

                    match status {
                        // ServerState::Running
                        0 => Some((SessionActivity::KeepAliveSucceeded, slf)),
                        s => {
                            warn!("Keep alive failed, non-running status code {s}");
                            Some((
                                SessionActivity::KeepAliveFailed(StatusCode::BadServerHalted),
                                slf,
                            ))
                        }
                    }
                }
            }
        })
    }
}
