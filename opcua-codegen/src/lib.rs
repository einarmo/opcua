mod error;
mod types;
mod xml;

use error::CodeGenError;
use types::CodeGenerator;
pub use types::{
    base_ignored_types, base_json_serialized_types, base_native_type_mappings,
    basic_types_import_map, GeneratedItem, ItemDefinition, LoadedTypes, StructureField,
    StructureFieldType, StructuredType, TypeLoader,
};

pub fn default_type_loader<'a>(data: &'a str) -> Result<TypeLoader<'a>, CodeGenError<'a>> {
    TypeLoader::new(base_ignored_types(), base_native_type_mappings(), data)
}

pub fn default_code_generator() -> CodeGenerator {
    CodeGenerator::new(base_json_serialized_types())
}
