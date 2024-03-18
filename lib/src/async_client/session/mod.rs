mod client;
mod connect;
mod event_loop;
mod services;
mod session;

use crate::client::prelude::{EndpointDescription, IdentityToken};

/// Information about the server endpoint, security policy, security mode and user identity that the session will
/// will use to establish a connection.
#[derive(Debug, Clone)]
pub struct SessionInfo {
    /// The endpoint
    pub endpoint: EndpointDescription,
    /// User identity token
    pub user_identity_token: IdentityToken,
    /// Preferred language locales
    pub preferred_locales: Vec<String>,
}

impl From<EndpointDescription> for SessionInfo {
    fn from(value: EndpointDescription) -> Self {
        Self {
            endpoint: value,
            user_identity_token: IdentityToken::Anonymous,
            preferred_locales: Vec::new(),
        }
    }
}

impl From<(EndpointDescription, IdentityToken)> for SessionInfo {
    fn from(value: (EndpointDescription, IdentityToken)) -> Self {
        Self {
            endpoint: value.0,
            user_identity_token: value.1,
            preferred_locales: Vec::new(),
        }
    }
}

pub use client::AsyncClient;
pub use event_loop::SessionEventLoop;
pub use services::subscriptions::{OnSubscriptionNotification, SubscriptionCallbacks};
pub use session::AsyncSession;

#[allow(unused)]
macro_rules! session_warn {
    ($session: expr, $($arg:tt)*) =>  {
        warn!("session:{} {}", $session.session_id(), format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use session_warn;

#[allow(unused)]
macro_rules! session_error {
    ($session: expr, $($arg:tt)*) =>  {
        error!("session:{} {}", $session.session_id(), format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use session_error;

#[allow(unused)]
macro_rules! session_debug {
    ($session: expr, $($arg:tt)*) =>  {
        debug!("session:{} {}", $session.session_id(), format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use session_debug;

#[allow(unused)]
macro_rules! session_trace {
    ($session: expr, $($arg:tt)*) =>  {
        trace!("session:{} {}", $session.session_id(), format!($($arg)*));
    }
}
#[allow(unused)]
pub(crate) use session_trace;
