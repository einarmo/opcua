use std::{future::Future, sync::Arc};

use opcua_types::StatusCode;

use crate::info::ServerInfo;

use super::tcp::TcpTransport;

pub trait Connector {
    fn connect(
        self,
        info: Arc<ServerInfo>,
    ) -> impl Future<Output = Result<TcpTransport, StatusCode>> + Send + Sync;
}
