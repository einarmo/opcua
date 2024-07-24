mod error;
mod types;
mod utils;

use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

use error::CodeGenError;
use opcua_xml::load_bsd_file;
use serde::{Deserialize, Serialize};
pub use types::{
    base_ignored_types, base_json_serialized_types, base_native_type_mappings,
    basic_types_import_map, BsdTypeLoader, CodeGenItemConfig, GeneratedItem, ItemDefinition,
    LoadedType, LoadedTypes, StructureField, StructureFieldType, StructuredType,
};
use types::{CodeGenerator, ExternalType};
pub use utils::{create_module_file, GeneratedOutput};

pub fn write_to_directory<T: GeneratedOutput>(
    dir: &str,
    items: impl Iterator<Item = T>,
) -> Result<(), CodeGenError> {
    let mut modules = HashSet::new();
    std::fs::remove_dir_all(dir)?;
    std::fs::create_dir_all(dir)?;

    for gen in items {
        let module = gen.module().to_owned();
        let mut file = std::fs::File::options()
            .append(true)
            .create(true)
            .open(format!("{}/{}.rs", dir, module))?;
        modules.insert(module);
        file.write_all(&prettyplease::unparse(&gen.to_file()).as_bytes())?;
    }

    let mut mod_file = std::fs::File::options()
        .append(true)
        .create(true)
        .open(format!("{}/{}", dir, "mod.rs"))?;
    let module_file = create_module_file(modules);
    mod_file.write_all(&prettyplease::unparse(&module_file).as_bytes())?;

    Ok(())
}

pub fn generate_types(
    config: &CodeGenConfig,
    target: &TypeCodeGenTarget,
) -> Result<Vec<GeneratedItem>, CodeGenError> {
    let path = std::path::Path::new(&target.file_path);
    let data = std::fs::read_to_string(&target.file_path)?;
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
        CodeGenItemConfig {
            enums_single_file: target.enums_single_file,
            structs_single_file: target.structs_single_file,
            opcua_crate_path: config.opcua_crate_path.clone(),
        },
    );

    generator.generate_types()
}

pub fn run_codegen(config: &CodeGenConfig) -> Result<(), CodeGenError> {
    for target in &config.targets {
        match target {
            CodeGenTarget::Types(t) => {
                println!("Running data type code generation for {}", t.file_path);
                let types = generate_types(config, t)?;
                println!("Writing {} types to {}", types.len(), t.output_dir);
                write_to_directory(&t.output_dir, types.into_iter())?;
            }
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TypeCodeGenTarget {
    pub file_path: String,
    pub output_dir: String,
    #[serde(default)]
    pub ignore: Vec<String>,
    #[serde(default)]
    pub json_serialized_types: Vec<String>,
    #[serde(default)]
    pub types_import_map: HashMap<String, ExternalType>,
    #[serde(default)]
    pub enums_single_file: bool,
    #[serde(default)]
    pub structs_single_file: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum CodeGenTarget {
    Types(TypeCodeGenTarget),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeGenConfig {
    pub extra_header: String,
    pub opcua_crate_path: String,
    pub targets: Vec<CodeGenTarget>,
}
