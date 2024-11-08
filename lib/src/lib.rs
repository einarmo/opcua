#[macro_use]
extern crate log;
#[cfg(test)]
extern crate serde_json;
#[cfg(test)]
extern crate tempdir;

pub use opcua_core::sync;

#[cfg(feature = "xml")]
pub use opcua_macros::FromXml;
#[cfg(feature = "server")]
pub use opcua_macros::{Event, EventField};

#[cfg(feature = "client")]
pub use opcua_client as client;
#[cfg(feature = "console-logging")]
pub mod console_logging;
#[cfg(feature = "server")]
pub use opcua_nodes as nodes;
#[cfg(feature = "server")]
pub use opcua_server as server;

pub use opcua_core as core;
pub use opcua_crypto as crypto;
pub use opcua_types as types;

#[cfg(feature = "generated-address-space")]
pub use opcua_core_namespace as core_namespace;
