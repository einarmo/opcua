mod error;
mod types;
mod utils;
mod xml;

use error::CodeGenError;
use serde::{Deserialize, Serialize};
pub use types::{
    base_ignored_types, base_json_serialized_types, base_native_type_mappings,
    basic_types_import_map, CodeGenItemConfig, GeneratedItem, ItemDefinition, LoadedTypes,
    StructureField, StructureFieldType, StructuredType, TypeLoader,
};
use types::{CodeGenerator, LoadedType};
pub use utils::create_module_file;

pub fn default_type_loader<'a>(data: &'a str) -> Result<TypeLoader<'a>, CodeGenError<'a>> {
    TypeLoader::new(base_ignored_types(), base_native_type_mappings(), data)
}

pub fn default_code_generator(
    loaded_types: Vec<LoadedType>,
    config: CodeGenItemConfig,
) -> CodeGenerator {
    CodeGenerator::new(
        base_json_serialized_types(),
        basic_types_import_map(&config.opcua_crate_path),
        loaded_types,
        config,
    )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeGenTarget {
    pub file_path: String,
    pub output_dir: String,
    pub enums_single_file: bool,
    pub structs_single_file: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeGenConfig {
    pub extra_header: String,
    pub opcua_crate_path: String,
}
