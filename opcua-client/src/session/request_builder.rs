use std::{future::Future, time::Duration};

use opcua_types::{DateTime, DiagnosticBits, IntegerId, NodeId, RequestHeader, StatusCode};

use crate::AsyncSecureChannel;

use super::Session;

pub trait UARequest {
    type Out;

    fn send<'a>(
        self,
        channel: &'a AsyncSecureChannel,
    ) -> impl Future<Output = Result<Self::Out, StatusCode>> + Send + Sync + 'a
    where
        Self: 'a;
}

#[derive(Debug, Clone)]
pub(crate) struct RequestHeaderBuilder {
    pub(crate) header: RequestHeader,
    pub(crate) timeout: Duration,
    pub(crate) session_id: u32,
}

impl RequestHeaderBuilder {
    pub fn new_from_session(session: &Session) -> Self {
        Self {
            header: session.make_request_header(),
            timeout: session.request_timeout,
            session_id: session.session_id(),
        }
    }

    pub fn new(
        session_id: u32,
        timeout: Duration,
        auth_token: NodeId,
        request_handle: IntegerId,
    ) -> Self {
        Self {
            header: RequestHeader {
                authentication_token: auth_token,
                timestamp: DateTime::now(),
                request_handle,
                return_diagnostics: DiagnosticBits::empty(),
                timeout_hint: timeout.as_millis().min(u32::MAX as u128) as u32,
                ..Default::default()
            },
            timeout,
            session_id,
        }
    }
}

macro_rules! builder_base {
    ($st:ident) => {
        impl $st {
            builder_base!(!_inner);
        }
    };

    ($st:ident$($gen:tt)*) => {
        impl$($gen)* $st$($gen)* {
            builder_base!(!_inner);
        }
    };

    (!_inner) => {
        /// Set requested diagnostic bits.
        pub fn diagnostics(mut self, bits: opcua_types::DiagnosticBits) -> Self {
            self.header.header.return_diagnostics = bits;
            self
        }

        /// Set the timeout for this request. Defaults to session timeout.
        pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
            self.header.header.timeout_hint = timeout.as_millis().min(u32::MAX as u128) as u32;
            self.header.timeout = timeout;
            self
        }

        pub fn audit_entry_id(mut self, entry: impl Into<opcua_types::UAString>) -> Self {
            self.header.header.audit_entry_id = entry.into();
            self
        }

        pub fn header(&self) -> &opcua_types::RequestHeader {
            &self.header.header
        }
    }
}

pub(crate) use builder_base;

#[allow(unused)]
macro_rules! builder_warn {
    ($session: expr, $($arg:tt)*) =>  {
        log::warn!("session:{} {}", $session.header.session_id, format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use builder_warn;

#[allow(unused)]
macro_rules! builder_error {
    ($session: expr, $($arg:tt)*) =>  {
        log::error!("session:{} {}", $session.header.session_id, format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use builder_error;

#[allow(unused)]
macro_rules! builder_debug {
    ($session: expr, $($arg:tt)*) =>  {
        log::debug!("session:{} {}", $session.header.session_id, format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use builder_debug;

#[allow(unused)]
macro_rules! builder_trace {
    ($session: expr, $($arg:tt)*) =>  {
        log::trace!("session:{} {}", $session.header.session_id, format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use builder_trace;
