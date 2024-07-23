mod error;
mod types;
mod utils;
mod xml;

use error::CodeGenError;
pub use types::{
    base_ignored_types, base_json_serialized_types, base_native_type_mappings,
    basic_types_import_map, GeneratedItem, ItemDefinition, LoadedTypes, StructureField,
    StructureFieldType, StructuredType, TypeLoader,
};
use types::{make_import_lookup_map, CodeGenerator, LoadedType};
pub use utils::create_module_file;

pub fn default_type_loader<'a>(data: &'a str) -> Result<TypeLoader<'a>, CodeGenError<'a>> {
    TypeLoader::new(base_ignored_types(), base_native_type_mappings(), data)
}

pub fn default_code_generator(
    loaded_types: Vec<LoadedType>,
    base_types_path_root: &str,
) -> CodeGenerator {
    CodeGenerator::new(
        base_json_serialized_types(),
        make_import_lookup_map(basic_types_import_map(), base_types_path_root),
        loaded_types,
        "opcua",
    )
}
