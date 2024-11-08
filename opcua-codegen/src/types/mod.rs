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

use crate::{CodeGenError, TypeCodeGenTarget, BASE_NAMESPACE};

pub fn generate_types(
    target: &TypeCodeGenTarget,
    root_path: &str,
) -> Result<(Vec<GeneratedItem>, String), CodeGenError> {
    println!("Loading types from {}", target.file_path);
    let data = std::fs::read_to_string(format!("{}/{}", root_path, &target.file_path))
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
    let target_namespace = type_loader.target_namespace();
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
        target_namespace.clone(),
    );

    Ok((generator.generate_types()?, target_namespace))
}

pub fn generate_xml_loader_impl(ids: HashMap<String, String>, namespace: &str) -> Vec<Item> {
    let mut ids: Vec<_> = ids.into_iter().collect();
    ids.sort_by(|a, b| a.1.cmp(&b.1));
    let mut fields = quote! {};
    for (field, typ) in ids {
        let field_ident = Ident::new(&field, Span::call_site());
        let typ_ident = Ident::new(&typ, Span::call_site());
        fields.extend(quote! {
            crate::ObjectId::#field_ident => #typ_ident::from_xml(body, ctx)
                .map(|v| opcua::types::ExtensionObject::from_message_full(&v, ctx.ns_map())),
        });
    }

    let index_check = if namespace != BASE_NAMESPACE {
        quote! {
            let idx = ctx.namespaces.namespaces().get_index(#namespace)?;
            if idx != node_id.namespace {
                return None;
            }
        }
    } else {
        quote! {
            if node_id.namespace != 0 {
                return None;
            }
        }
    };

    let mut items = Vec::new();
    items.push(Item::Struct(parse_quote! {
        #[cfg(feature = "xml")]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct TypesXmlLoader;
    }));
    items.push(Item::Impl(parse_quote! {
        #[cfg(feature = "xml")]
        impl opcua::types::xml::XmlLoader for TypesXmlLoader {
            fn load_extension_object(
                &self,
                body: &opcua::types::xml::XmlElement,
                node_id: &opcua::types::NodeId,
                ctx: &opcua::types::xml::XmlContext<'_>
            ) -> Option<Result<opcua::types::ExtensionObject, opcua::types::xml::FromXmlError>> {
                use opcua::types::xml::FromXml;

                #index_check

                let object_id = match node_id
                    .as_u32()
                    .and_then(|v| crate::ObjectId::try_from(v).ok())
                    .ok_or_else(|| format!("Invalid object ID: {node_id}"))
                {
                    Ok(i) => i,
                    Err(e) => return Some(Err(e.into()))
                };
                let r = match object_id {
                    #fields
                    _ => return None,
                };

                match r {
                    Ok(r) => Some(r.map_err(|_| {
                        opcua::types::xml::FromXmlError::from(format!(
                            "Invalid XML type, missing binary encoding ID: {:?}",
                            object_id
                        ))
                    })),
                    Err(e) => Some(Err(e)),
                }
            }
        }
    }));
    items
}
