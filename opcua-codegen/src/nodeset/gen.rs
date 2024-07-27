use std::collections::HashMap;

use opcua_xml::schema::ua_node_set::{
    AliasTable, ArrayDimensions, LocalizedText, NodeId, Reference, UADataType, UAMethod, UANode,
    UANodeBase, UAObject, UAObjectType, UAReferenceType, UAVariable, UAVariableType, UAView,
};
use proc_macro2::TokenStream;
use syn::{parse_quote, parse_str, Expr, Ident, ItemFn, Path};

use crate::{utils::RenderExpr, CodeGenError};

use super::{value::render_value, XsdTypeWithPath};

pub struct NodeGenMethod {
    pub func: ItemFn,
    pub name: String,
}

pub struct NodeSetCodeGenerator {
    opcua_path: Path,
    opcua_path_str: String,
    preferred_locale: String,
    empty_text: LocalizedText,
    aliases: HashMap<String, String>,
    node_counter: usize,
    types: HashMap<String, XsdTypeWithPath>,
}

impl NodeSetCodeGenerator {
    pub fn new(
        opcua_path_str: &str,
        preferred_locale: &str,
        alias_table: Option<AliasTable>,
        types: HashMap<String, XsdTypeWithPath>,
    ) -> Result<Self, CodeGenError> {
        let mut aliases = HashMap::new();
        if let Some(alias_table) = alias_table {
            for alias in alias_table.aliases {
                aliases.insert(alias.alias, alias.id.0);
            }
        }
        let opcua_path: Path = parse_str(opcua_path_str)?;

        Ok(Self {
            opcua_path,
            opcua_path_str: opcua_path_str.to_owned(),
            preferred_locale: preferred_locale.to_owned(),
            empty_text: LocalizedText::default(),
            aliases,
            node_counter: 0,
            types,
        })
    }

    fn resolve_node_id(&self, node_id: &NodeId) -> Result<TokenStream, CodeGenError> {
        if let Some(aliased) = self.aliases.get(&node_id.0) {
            NodeId(aliased.to_owned()).render(&self.opcua_path)
        } else {
            node_id.render(&self.opcua_path)
        }
    }

    fn get_localized_text<'a: 'b, 'b>(&'a self, options: &'b [LocalizedText]) -> &'b LocalizedText {
        options
            .iter()
            .find(|f| f.locale.0 == self.preferred_locale)
            .or_else(|| options.first())
            .unwrap_or_else(|| &self.empty_text)
    }

    fn get_localized_text_opt<'a: 'b, 'b>(
        &'a self,
        options: &'b [LocalizedText],
    ) -> Option<&'b LocalizedText> {
        options
            .iter()
            .find(|f| f.locale.0 == self.preferred_locale)
            .or_else(|| options.first())
    }

    fn parse_array_dimensions(
        &self,
        dims: &ArrayDimensions,
    ) -> Result<Option<Vec<u32>>, CodeGenError> {
        if dims.0.trim().is_empty() {
            return Ok(None);
        }

        let mut values = Vec::with_capacity(1);
        for it in dims.0.split(',') {
            values.push(it.trim().parse::<u32>().map_err(|_| {
                CodeGenError::Other(format!("Invalid array dimensions: {}", dims.0))
            })?);
        }

        Ok(Some(values))
    }

    fn generate_base(&self, node: &UANodeBase, node_class: &str) -> Result<Expr, CodeGenError> {
        let name = self
            .get_localized_text(&node.display_names)
            .render(&self.opcua_path)?;
        let description = self
            .get_localized_text_opt(&node.description)
            .render(&self.opcua_path)?;
        let browse_name = node.browse_name.render(&self.opcua_path)?;
        let node_class: Expr = syn::parse_str(&format!(
            "{}::types::NodeClass::{}",
            self.opcua_path_str, node_class
        ))?;
        let write_mask = node.write_mask.0;
        let user_write_mask = node.user_write_mask.0;
        let node_id = self.resolve_node_id(&node.node_id)?;

        let opcua_path = &self.opcua_path;
        Ok(parse_quote! {
            #opcua_path::server::address_space::Base::new_full(
                #node_id, #node_class, #browse_name, #name, #description,
                Some(#write_mask), Some(#user_write_mask)
            )
        })
    }

    fn generate_object(&self, node: &UAObject) -> Result<Expr, CodeGenError> {
        let base = self.generate_base(&node.base.base, "Object")?;
        let opcua_path = &self.opcua_path;
        let event_notifier = node.event_notifier.0;
        Ok(parse_quote! {
            #opcua_path::server::address_space::Object::new_full(
                #base,
                #opcua_path::server::address_space::EventNotifier::from_bits_truncate(#event_notifier),
            )
        })
    }

    fn generate_variable(&self, node: &UAVariable) -> Result<Expr, CodeGenError> {
        let base = self.generate_base(&node.base.base, "Variable")?;
        let opcua_path = &self.opcua_path;
        let data_type = self.resolve_node_id(&node.data_type)?;
        let historizing = node.historizing;
        let value_rank = node.value_rank.0;
        let value = render_value(node.value.as_ref(), opcua_path, &self.types)?;
        let access_level = node.access_level.0;
        let user_access_level = node.user_access_level.0;
        let array_dimensions = self
            .parse_array_dimensions(&node.array_dimensions)?
            .as_ref()
            .render(&self.opcua_path)?;
        let minimum_sampling_interval =
            node.minimum_sampling_interval.0.render(&self.opcua_path)?;

        Ok(parse_quote! {
            #opcua_path::server::address_space::Variable::new_full(
                #base,
                #data_type,
                #historizing,
                #value_rank,
                #value,
                #access_level,
                #user_access_level,
                #array_dimensions,
                Some(#minimum_sampling_interval),
            )
        })
    }

    fn generate_method(&self, node: &UAMethod) -> Result<Expr, CodeGenError> {
        let base = self.generate_base(&node.base.base, "Method")?;
        let executable = node.executable;
        let user_executable = node.user_executable;
        let opcua_path = &self.opcua_path;

        Ok(parse_quote! {
            #opcua_path::server::address_space::Method::new_full(
                #base,
                #executable,
                #user_executable,
            )
        })
    }

    fn generate_view(&self, node: &UAView) -> Result<Expr, CodeGenError> {
        let base = self.generate_base(&node.base.base, "View")?;
        let opcua_path = &self.opcua_path;
        let event_notifier = node.event_notifier.0;
        let contains_no_loops = node.contains_no_loops;

        Ok(parse_quote! {
            #opcua_path::server::address_space::View::new_full(
                #base,
                #opcua_path::server::address_space::EventNotifier::from_bits_truncate(#event_notifier),
                #contains_no_loops,
            )
        })
    }

    fn generate_object_type(&self, node: &UAObjectType) -> Result<Expr, CodeGenError> {
        let base = self.generate_base(&node.base.base, "ObjectType")?;
        let opcua_path = &self.opcua_path;
        let is_abstract = node.base.is_abstract;

        Ok(parse_quote! {
            #opcua_path::server::address_space::ObjectType::new_full(
                #base,
                #is_abstract,
            )
        })
    }

    fn generate_variable_type(&self, node: &UAVariableType) -> Result<Expr, CodeGenError> {
        let base = self.generate_base(&node.base.base, "VariableType")?;
        let opcua_path = &self.opcua_path;
        let data_type = self.resolve_node_id(&node.data_type)?;
        let is_abstract = node.base.is_abstract;
        let value_rank = node.value_rank.0;
        let value = render_value(node.value.as_ref(), opcua_path, &self.types)?;
        let array_dimensions = self
            .parse_array_dimensions(&node.array_dimensions)?
            .as_ref()
            .render(&self.opcua_path)?;
        Ok(parse_quote! {
            #opcua_path::server::address_space::VariableType::new_full(
                #base,
                #data_type,
                #is_abstract,
                #value_rank,
                Some(#value),
                #array_dimensions,
            )
        })
    }

    fn generate_data_type(&self, node: &UADataType) -> Result<Expr, CodeGenError> {
        let base = self.generate_base(&node.base.base, "DataType")?;
        let is_abstract = node.base.is_abstract;
        let opcua_path = &self.opcua_path;

        Ok(parse_quote! {
            #opcua_path::server::address_space::DataType::new_full(
                #base,
                #is_abstract
            )
        })
    }

    fn generate_reference_type(&self, node: &UAReferenceType) -> Result<Expr, CodeGenError> {
        let base = self.generate_base(&node.base.base, "ReferenceType")?;
        let symmetric = node.symmetric;
        let is_abstract = node.base.is_abstract;
        let inverse_name = self
            .get_localized_text_opt(&node.inverse_names)
            .render(&self.opcua_path)?;
        let opcua_path = &self.opcua_path;

        Ok(parse_quote! {
            #opcua_path::server::address_space::ReferenceType::new_full(
                #base,
                #symmetric,
                #is_abstract,
                #inverse_name,
            )
        })
    }

    fn generate_reference(&self, reference: &Reference) -> Result<Expr, CodeGenError> {
        let target_id = self.resolve_node_id(&reference.node_id)?;
        let type_id = self.resolve_node_id(&reference.reference_type)?;
        let is_forward = reference.is_forward;

        let opcua_path = &self.opcua_path;
        Ok(parse_quote! {
            #opcua_path::server::address_space::ImportedReference {
                target_id: #target_id,
                type_id: #type_id,
                is_forward: #is_forward,
            }
        })
    }

    fn generate_references(&self, node: &UANodeBase) -> Result<Vec<Expr>, CodeGenError> {
        node.references
            .iter()
            .flat_map(|f| f.references.iter())
            .map(|r| self.generate_reference(r))
            .collect()
    }

    pub fn generate_item(&mut self, node: &UANode) -> Result<NodeGenMethod, CodeGenError> {
        let name = match node {
            UANode::Object(_) => "object",
            UANode::Variable(_) => "variable",
            UANode::Method(_) => "method",
            UANode::View(_) => "view",
            UANode::ObjectType(_) => "object_type",
            UANode::VariableType(_) => "variable_type",
            UANode::DataType(_) => "data_type",
            UANode::ReferenceType(_) => "reference_type",
        };
        let func_name_str = format!("make_{}_{}", name, self.node_counter);
        let func_name: Ident = parse_str(&func_name_str)?;
        self.node_counter += 1;
        let opcua_path = &self.opcua_path;

        let references = self.generate_references(node.base())?;
        let node = match &node {
            UANode::Object(n) => self.generate_object(n),
            UANode::Variable(n) => self.generate_variable(n),
            UANode::Method(n) => self.generate_method(n),
            UANode::View(n) => self.generate_view(n),
            UANode::ObjectType(n) => self.generate_object_type(n),
            UANode::VariableType(n) => self.generate_variable_type(n),
            UANode::DataType(n) => self.generate_data_type(n),
            UANode::ReferenceType(n) => self.generate_reference_type(n),
        }?;

        let func: ItemFn = parse_quote! {
            #[allow(unused)]
            fn #func_name(ns_map: &#opcua_path::server::address_space::NodeSetNamespaceMapper<'_>)
                -> #opcua_path::server::address_space::ImportedItem
            {
                #opcua_path::server::address_space::ImportedItem {
                    node: #node.into(),
                    references: vec![#(#references),*]
                }
            }
        };

        Ok(NodeGenMethod {
            func,
            name: func_name_str,
        })
    }
}
