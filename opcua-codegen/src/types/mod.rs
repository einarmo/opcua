mod base_constants;
mod enum_type;
#[cfg(feature = "codegen")]
mod gen;
mod loader;
mod structure;

pub use base_constants::*;
pub use enum_type::{EnumType, EnumValue};
pub use gen::{CodeGenItemConfig, CodeGenerator, GeneratedItem, ItemDefinition};
pub use loader::{BsdTypeLoader, LoadedType, LoadedTypes};
use opcua_xml::load_bsd_file;
pub use structure::{StructureField, StructureFieldType, StructuredType};

use crate::{CodeGenConfig, CodeGenError, TypeCodeGenTarget};

pub fn generate_types(
    config: &CodeGenConfig,
    target: &TypeCodeGenTarget,
) -> Result<Vec<GeneratedItem>, CodeGenError> {
    let path = std::path::Path::new(&target.file_path);
    let data = std::fs::read_to_string(&target.file_path)
        .map_err(|e| CodeGenError::io(&format!("Failed to read file {}", target.file_path), e))?;
    let type_dictionary = load_bsd_file(&data)?;

    let types = match path.extension().and_then(|p| p.to_str()) {
        Some("bsd") => {
            let type_loader = BsdTypeLoader::new(
                target
                    .ignore
                    .iter()
                    .cloned()
                    .chain(base_ignored_types().into_iter())
                    .collect(),
                base_native_type_mappings(),
                type_dictionary,
            )?;
            type_loader.from_bsd()?
        }
        Some(r) => {
            return Err(CodeGenError::Other(format!(
                "Invalid code gen file, unknown extension {r}"
            )))
        }
        None => {
            return Err(CodeGenError::Other(
                "Invalid code gen file, no extension".to_owned(),
            ))
        }
    };

    let mut types_import_map = basic_types_import_map(&config.opcua_crate_path);
    for (k, v) in &target.types_import_map {
        types_import_map.insert(k.clone(), v.clone());
    }

    let generator = CodeGenerator::new(
        target
            .json_serialized_types
            .iter()
            .cloned()
            .chain(base_json_serialized_types().into_iter())
            .collect(),
        types_import_map,
        types,
        target.default_excluded.clone(),
        CodeGenItemConfig {
            enums_single_file: target.enums_single_file,
            structs_single_file: target.structs_single_file,
            opcua_crate_path: config.opcua_crate_path.clone(),
        },
    );

    generator.generate_types()
}
