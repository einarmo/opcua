//! Tools for working with structs whose structure is not known at compile time,
//! using [`crate::DataTypeDefinition`] to encode and decode values.

mod custom_struct;
#[cfg(feature = "json")]
mod json;
mod type_tree;

pub use custom_struct::{DynamicStructure, DynamicTypeLoader};
pub use type_tree::{
    DataTypeTree, EncodingIds, EnumTypeInfo, ParentIds, ParsedStructureField, StructTypeInfo,
};
