//! Implementation of [AddressSpace], and in-memory OPC-UA address space.

mod implementation;
mod utils;

pub use implementation::{AddressSpace, Reference, ReferenceRef};
pub use opcua_nodes::*;
pub use utils::*;

#[cfg(feature = "generated-address-space")]
pub use opcua_core_namespace::CoreNamespace;
