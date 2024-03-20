use std::{sync::Arc, time::Instant};

use futures::{future::Either, stream::FuturesUnordered, Future, Stream, StreamExt};

use crate::{
    async_client::{session::session_error, AsyncSession},
    client::prelude::StatusCode,
};

#[derive(Debug)]
pub enum SubscriptionActivity {
    Publish,
    PublishFailed(StatusCode),
}

pub struct SubscriptionEventLoop {
    session: Arc<AsyncSession>,
    trigger_publish_recv: tokio::sync::watch::Receiver<Instant>,
    max_inflight_publish: usize,
    last_external_trigger: Instant,
}

impl SubscriptionEventLoop {
    pub fn new(
        session: Arc<AsyncSession>,
        trigger_publish_recv: tokio::sync::watch::Receiver<Instant>,
    ) -> Self {
        let last_external_trigger = trigger_publish_recv.borrow().clone();
        Self {
            max_inflight_publish: session.max_inflight_publish,
            last_external_trigger,
            trigger_publish_recv,
            session,
        }
    }

    pub fn run(self) -> impl Stream<Item = SubscriptionActivity> {
        futures::stream::unfold(
            (self, FuturesUnordered::new()),
            |(mut slf, mut futures)| async move {
                let mut next = slf.session.next_publish_time(false);
                let mut recv: tokio::sync::watch::Receiver<Instant> =
                    slf.trigger_publish_recv.clone();

                let res = loop {
                    let next_tick_fut = if let Some(next) = next {
                        Either::Left(tokio::time::sleep_until(next.into()))
                    } else {
                        Either::Right(futures::future::pending::<()>())
                    };
                    let next_publish_fut = if futures.is_empty() {
                        Either::Left(futures::future::pending())
                    } else {
                        Either::Right(futures.next())
                    };

                    tokio::select! {
                        v = recv.wait_for(|i| i > &slf.last_external_trigger) => {
                            if let Ok(v) = v {
                                debug!("Sending publish due to external trigger");
                                futures.push(slf.static_publish());
                                next = slf.session.next_publish_time(true);
                                slf.last_external_trigger = v.clone();
                            }
                        }
                        _ = next_tick_fut => {
                            if futures.len() < slf.max_inflight_publish {
                                debug!("Sending publish due to internal tick");
                                futures.push(slf.static_publish());
                            }
                            next = slf.session.next_publish_time(true);
                        }
                        res = next_publish_fut => {
                            match res {
                                Some(Ok(should_publish_now)) => {
                                    if should_publish_now {
                                        futures.push(slf.static_publish());
                                        // Set the last publish time.
                                        // We do this to avoid a buildup of publish requests
                                        // if exhausting the queue takes more time than
                                        // a single publishing interval.
                                        slf.session.next_publish_time(true);
                                    }

                                    break SubscriptionActivity::Publish
                                }
                                Some(Err(e)) => {
                                    session_error!(slf.session, "Publish failed, sending a new request");
                                    if futures.len() < slf.max_inflight_publish {
                                        futures.push(slf.static_publish());
                                    }
                                    break SubscriptionActivity::PublishFailed(e)
                                }
                                // Should be impossible
                                None => break SubscriptionActivity::PublishFailed(StatusCode::BadInvalidState)
                            }
                        }
                    }
                };

                Some((res, (slf, futures)))
            },
        )
    }

    fn static_publish(&self) -> impl Future<Output = Result<bool, StatusCode>> + 'static {
        let inner_session = self.session.clone();
        async move { inner_session.publish().await }
    }
}
