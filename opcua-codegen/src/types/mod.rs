mod base_constants;
mod enum_type;
mod gen;
mod loader;
mod structure;

use std::collections::HashMap;

pub use base_constants::*;
pub use enum_type::{EnumType, EnumValue};
pub use gen::{CodeGenItemConfig, CodeGenerator, GeneratedItem, ItemDefinition};
pub use loader::{BsdTypeLoader, LoadedType, LoadedTypes};
use opcua_xml::load_bsd_file;
use proc_macro2::Span;
use quote::quote;
pub use structure::{StructureField, StructureFieldType, StructuredType};
use syn::{parse_quote, Ident, Item};

use crate::{CodeGenError, TypeCodeGenTarget};

pub fn generate_types(target: &TypeCodeGenTarget) -> Result<Vec<GeneratedItem>, CodeGenError> {
    println!("Loading types from {}", target.file_path);
    let data = std::fs::read_to_string(&target.file_path)
        .map_err(|e| CodeGenError::io(&format!("Failed to read file {}", target.file_path), e))?;
    let type_dictionary = load_bsd_file(&data)?;
    println!(
        "Found {} raw elements in the type dictionary.",
        type_dictionary.elements.len()
    );
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
    let types = type_loader.from_bsd()?;
    println!("Generated code for {} types", types.len());

    let mut types_import_map = basic_types_import_map();
    for (k, v) in &target.types_import_map {
        types_import_map.insert(k.clone(), v.clone());
    }

    let generator = CodeGenerator::new(
        types_import_map,
        types,
        target.default_excluded.clone(),
        CodeGenItemConfig {
            enums_single_file: target.enums_single_file,
            structs_single_file: target.structs_single_file,
        },
    );

    generator.generate_types()
}

pub fn generate_xml_loader_impl(ids: HashMap<String, String>) -> Vec<Item> {
    let mut fields = quote! {};
    for (field, typ) in ids {
        let field_ident = Ident::new(&field, Span::call_site());
        let typ_ident = Ident::new(&typ, Span::call_site());
        fields.extend(quote! {
            opcua::types::ObjectId::#field_ident => #typ_ident::from_xml(body, ctx)
                .map(|v| opcua::types::ExtensionObject::from_message(&v)),
        });
    }
    let mut items = Vec::new();
    items.push(Item::Struct(parse_quote! {
        #[cfg(feature = "xml")]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct TypesXmlLoader;
    }));
    items.push(Item::Impl(parse_quote! {
        #[cfg(feature = "xml")]
        impl opcua::types::xml::XmlLoader for TypesXmlLoader {
            fn load_extension_object<'a>(
                &self,
                body: &opcua::types::xml::XmlElement,
                node_id: &opcua::types::NodeId,
                ctx: &opcua::types::xml::XmlContext<'a>
            ) -> Option<Result<opcua::types::ExtensionObject, opcua::types::xml::FromXmlError>> {
                use opcua::types::xml::FromXml;

                let object_id = match node_id.as_object_id().map_err(|_| "Invalid object ID".to_owned()) {
                    Ok(i) => i,
                    Err(e) => return Some(Err(e.into()))
                };
                Some(match object_id {
                    #fields
                    _ => return None,
                })
            }
        }
    }));
    items
}
