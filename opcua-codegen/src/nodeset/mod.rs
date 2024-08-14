mod gen;
mod render;
mod value;

use std::collections::HashMap;

pub use gen::{NodeGenMethod, NodeSetCodeGenerator};
use opcua_xml::{
    load_nodeset2_file,
    schema::xml_schema::{load_xsd_schema, XsdFileItem, XsdFileType},
};
use proc_macro2::Span;
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::{parse_quote, parse_str, File, Ident, Item, ItemFn, Path};

use crate::{CodeGenError, GeneratedOutput};

pub struct XsdTypeWithPath {
    pub ty: XsdFileType,
    pub path: Path,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NodeSetTypes {
    pub file_path: String,
    pub root_path: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NodeSetCodeGenTarget {
    pub file_path: String,
    pub output_dir: String,
    pub max_nodes_per_file: usize,
    pub types: Vec<NodeSetTypes>,
    pub own_namespaces: Vec<String>,
    pub imported_namespaces: Vec<String>,
    pub name: String,
}

pub fn make_type_dict(
    target: &NodeSetCodeGenTarget,
) -> Result<HashMap<String, XsdTypeWithPath>, CodeGenError> {
    let mut res = HashMap::new();
    for file in &target.types {
        let xsd_file = std::fs::read_to_string(&file.file_path).map_err(|e| {
            CodeGenError::io(&format!("Failed to read file {}", target.file_path), e)
        })?;
        let path: Path = parse_str(&file.root_path)?;
        let xsd_file = load_xsd_schema(&xsd_file)?;

        for it in xsd_file.items {
            let (ty, name) = match it {
                XsdFileItem::SimpleType(i) => {
                    if let Some(name) = i.name.clone() {
                        (XsdFileType::Simple(i), name)
                    } else {
                        continue;
                    }
                }
                XsdFileItem::ComplexType(i) => {
                    if let Some(name) = i.name.clone() {
                        (XsdFileType::Complex(i), name)
                    } else {
                        continue;
                    }
                }
                XsdFileItem::Element(_) => continue,
            };
            res.insert(
                name,
                XsdTypeWithPath {
                    ty,
                    path: path.clone(),
                },
            );
        }
    }
    Ok(res)
}

pub struct NodeSetChunk {
    pub root_fun: ItemFn,
    pub items: Vec<ItemFn>,
    pub name: String,
}

impl GeneratedOutput for NodeSetChunk {
    fn to_file(self) -> syn::File {
        let mut fns = Vec::new();
        fns.push(self.root_fun);
        fns.extend(self.items.into_iter());

        syn::File {
            shebang: None,
            attrs: Vec::new(),
            items: fns.into_iter().map(Item::Fn).collect(),
        }
    }

    fn module(&self) -> &str {
        &self.name
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub fn make_root_fun(chunk: &[NodeGenMethod], opcua_path: &Path) -> ItemFn {
    let mut names = chunk.iter().map(|c| Ident::new(&c.name, Span::call_site()));

    // Create a list of the functions, but as &dyn Fn, to make it easy to make an iterator.
    // Also return the value as a boxed dyn iterator, by doing it this way we don't get an
    // enormous type signature on the final iterator,
    // and the runtime cost of a little indirection is so small it doesn't matter.
    let first = names.next().unwrap();
    parse_quote! {
        pub(super) fn imported_nodes<'a>(ns_map: &'a #opcua_path::server::address_space::NodeSetNamespaceMapper<'_>) -> Box<dyn Iterator<
            Item = #opcua_path::server::address_space::ImportedItem
        > + 'a> {
            Box::new([
                &#first as &dyn Fn(_) -> #opcua_path::server::address_space::ImportedItem,
                #(&#names),*
            ].into_iter().map(|f| f(ns_map)))
        }
    }
}

pub fn generate_target(
    config: &NodeSetCodeGenTarget,
    opcua_path: &str,
    preferred_locale: &str,
) -> Result<Vec<NodeSetChunk>, CodeGenError> {
    println!("Loading node set from {}", config.file_path);
    let node_set = std::fs::read_to_string(&config.file_path)
        .map_err(|e| CodeGenError::io(&format!("Failed to read file {}", config.file_path), e))?;
    let node_set = load_nodeset2_file(&node_set)?;

    let nodes = node_set
        .node_set
        .ok_or_else(|| CodeGenError::Other("Missing UANodeSet in xml schema".to_owned()))?;
    println!("Found {} nodes in node set", nodes.nodes.len());

    let types = make_type_dict(&config)?;

    let mut generator =
        NodeSetCodeGenerator::new(opcua_path, preferred_locale, nodes.aliases, types)?;

    let mut fns = Vec::with_capacity(nodes.nodes.len());
    for node in &nodes.nodes {
        fns.push(generator.generate_item(node)?);
    }
    fns.sort_by(|a, b| a.name.cmp(&b.name));
    println!("Generated {} node creation methods", fns.len());

    let mut iter = fns.into_iter();

    let opcua_path = parse_str(opcua_path)?;

    let mut outputs = Vec::new();
    let mut chunk = Vec::new();
    while let Some(it) = iter.next() {
        chunk.push(it);
        if chunk.len() == config.max_nodes_per_file {
            outputs.push(NodeSetChunk {
                root_fun: make_root_fun(&chunk, &opcua_path),
                items: chunk.into_iter().map(|c| c.func).collect(),
                name: format!("nodeset_{}", outputs.len() + 1),
            });
            chunk = Vec::new();
        }
    }

    if !chunk.is_empty() {
        outputs.push(NodeSetChunk {
            root_fun: make_root_fun(&chunk, &opcua_path),
            items: chunk.into_iter().map(|c| c.func).collect(),
            name: format!("nodeset_{}", outputs.len() + 1),
        });
    }

    Ok(outputs)
}

pub fn make_root_module(
    chunks: &[NodeSetChunk],
    opcua_path: &str,
    config: &NodeSetCodeGenTarget,
) -> Result<File, CodeGenError> {
    let mut items: Vec<Item> = Vec::new();
    let mut names = Vec::new();
    for chunk in chunks {
        let ident = Ident::new(&chunk.name, Span::call_site());
        names.push(ident.clone());
        items.push(parse_quote! {
            mod #ident;
        });
    }

    let name_ident = Ident::new(&config.name, Span::call_site());

    items.push(parse_quote! {
        pub struct #name_ident;
    });

    let opcua_path: Path = parse_str(opcua_path)?;

    let mut namespace_adds = quote! {};
    for (idx, ns) in config
        .imported_namespaces
        .iter()
        .chain(config.own_namespaces.iter())
        .enumerate()
    {
        let idx = idx as u16;
        namespace_adds.extend(quote! {
            map.add_namespace(#ns, #idx);
        });
    }

    let mut namespace_out = quote! {};
    for ns in config.own_namespaces.iter() {
        namespace_out.extend(quote! {
            #ns.to_owned(),
        })
    }

    items.push(parse_quote! {
        impl #opcua_path::server::address_space::NodeSetImport for #name_ident {
            fn load<'a>(map: &'a #opcua_path::server::address_space::NodeSetNamespaceMapper) -> impl Iterator<Item = #opcua_path::server::address_space::ImportedItem> + 'a {
                [
                    #(#names::imported_nodes(map)),*
                ].into_iter().flat_map(|f| f)
            }

            fn register_namespaces(map: &mut #opcua_path::server::address_space::NodeSetNamespaceMapper) -> Vec<String> {
                #namespace_adds

                vec![#namespace_out]
            }
        }
    });

    Ok(File {
        attrs: Vec::new(),
        shebang: None,
        items,
    })
}
