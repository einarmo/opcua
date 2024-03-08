use std::time::Duration;

pub(crate) struct ExponentialBackoff {
    max_sleep: Duration,
    max_retries: Option<u32>,
    current_sleep: Duration,
    retry_count: u32,
}

impl ExponentialBackoff {
    pub fn new(max_sleep: Duration, max_retries: Option<u32>, initial_sleep: Duration) -> Self {
        log::info!("New backoff: {max_sleep:?} {max_retries:?} {initial_sleep:?}");
        Self {
            max_sleep,
            max_retries,
            current_sleep: initial_sleep,
            retry_count: 0,
        }
    }
}

impl Iterator for ExponentialBackoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max_retries.is_some_and(|max| max <= self.retry_count) {
            return None;
        }

        let next_sleep = self.current_sleep.clone();
        self.current_sleep = self.max_sleep.min(self.current_sleep * 2);
        self.retry_count += 1;

        Some(next_sleep)
    }
}

#[derive(Debug, Clone)]
pub struct SessionRetryPolicy {
    reconnect_max_sleep: Duration,
    reconnect_retry_limit: Option<u32>,
    reconnect_initial_sleep: Duration,

    keep_alive_interval: Duration,
}

impl SessionRetryPolicy {
    pub fn new(
        max_sleep: Duration,
        retry_limit: Option<u32>,
        initial_sleep: Duration,
        keep_alive_interval: Duration,
    ) -> Self {
        Self {
            reconnect_max_sleep: max_sleep,
            reconnect_retry_limit: retry_limit,
            reconnect_initial_sleep: initial_sleep,
            keep_alive_interval,
        }
    }

    pub(crate) fn new_backoff(&self) -> ExponentialBackoff {
        ExponentialBackoff::new(
            self.reconnect_max_sleep,
            self.reconnect_retry_limit,
            self.reconnect_initial_sleep,
        )
    }
}
