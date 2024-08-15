use std::collections::HashMap;

use convert_case::{Case, Casing};
use opcua_xml::schema::{
    opc_ua_types::{ExtensionObject, Variant, XmlElement},
    ua_node_set::Value,
    xml_schema::{
        ComplexContent, ComplexTypeContents, Element, Facet, FacetValue, MaxOccurs, NestedParticle,
        SimpleDerivation, TypeDefParticle, XsdFileType,
    },
};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Ident, Path};

use crate::{utils::RenderExpr, CodeGenError};

use super::XsdTypeWithPath;

macro_rules! from_vec {
    ($v:ident) => {
        quote::quote! {
            vec![#(#$v),*]
        }
    };
}

pub fn render_value(
    value: Option<&Value>,
    opcua_path: &Path,
    types: &HashMap<String, XsdTypeWithPath>,
) -> Result<TokenStream, CodeGenError> {
    ValueBuilder { opcua_path, types }.render_value(value)
}

struct ValueBuilder<'a> {
    types: &'a HashMap<String, XsdTypeWithPath>,
    opcua_path: &'a Path,
}

impl<'a> ValueBuilder<'a> {
    pub fn render_value(&self, value: Option<&Value>) -> Result<TokenStream, CodeGenError> {
        let opcua_path = self.opcua_path;

        if let Some(value) = value {
            let rendered = self.render_variant(&value.0)?;
            Ok(quote! {
                #opcua_path::types::DataValue::new_now(#rendered)
            })
        } else {
            Ok(quote! {
                #opcua_path::types::DataValue::null()
            })
        }
    }

    fn render_variant(&self, value: &Variant) -> Result<TokenStream, CodeGenError> {
        let opcua_path = self.opcua_path;

        let inner = match &value {
            Variant::Boolean(v) => v.to_token_stream(),
            Variant::ListOfBoolean(v) => from_vec!(v),
            Variant::SByte(v) => v.to_token_stream(),
            Variant::ListOfSByte(v) => from_vec!(v),
            Variant::Byte(v) => v.to_token_stream(),
            Variant::ListOfByte(v) => from_vec!(v),
            Variant::Int16(v) => v.to_token_stream(),
            Variant::ListOfInt16(v) => from_vec!(v),
            Variant::UInt16(v) => v.to_token_stream(),
            Variant::ListOfUInt16(v) => from_vec!(v),
            Variant::Int32(v) => v.to_token_stream(),
            Variant::ListOfInt32(v) => from_vec!(v),
            Variant::UInt32(v) => v.to_token_stream(),
            Variant::ListOfUInt32(v) => from_vec!(v),
            Variant::Int64(v) => v.to_token_stream(),
            Variant::ListOfInt64(v) => from_vec!(v),
            Variant::UInt64(v) => v.to_token_stream(),
            Variant::ListOfUInt64(v) => from_vec!(v),
            Variant::Float(v) => v.to_token_stream(),
            Variant::ListOfFloat(v) => from_vec!(v),
            Variant::Double(v) => v.to_token_stream(),
            Variant::ListOfDouble(v) => from_vec!(v),
            Variant::String(v) => v.to_token_stream(),
            Variant::ListOfString(v) => from_vec!(v),
            Variant::DateTime(v) => {
                let us = v.timestamp_micros();
                quote::quote! {
                    #opcua_path::types::DateTimeUtc::from_timestamp_micros(#us).unwrap()
                }
            }
            Variant::ListOfDateTime(v) => {
                let uss = v.iter().map(|v| v.timestamp_micros());
                quote::quote! {
                    vec![#(#opcua_path::types::DateTimeUtc::from_timestamp_micros(#uss).unwrap()),*]
                }
            }
            Variant::Guid(v) => {
                let bytes = v.as_bytes();
                quote::quote! {
                    #opcua_path::types::Uuid::from_slice(&[#(#bytes),*]).unwrap()
                }
            }
            Variant::ListOfGuid(v) => {
                let bytes = v.iter().map(|v| v.as_bytes());
                let mut items = quote::quote! {};
                for it in bytes {
                    items.extend(quote::quote! {
                        #items,
                        #opcua_path::types::Uuid::from_slice(&[#(#it),*]).unwrap(),
                    });
                }
                quote::quote! {
                    vec![#items]
                }
            }
            Variant::ByteString(v) => {
                let cleaned = v.replace('\n', "");
                quote::quote! {
                    #opcua_path::types::ByteString::from_base64(#cleaned).unwrap()
                }
            }
            Variant::ListOfByteString(v) => {
                let cleaned = v.iter().map(|v| v.replace('\n', ""));
                quote::quote! {
                    #(#opcua_path::types::ByteString::from_base64(#cleaned).unwrap()),*
                }
            }
            Variant::XmlElement(_) | Variant::ListOfXmlElement(_) => {
                println!("XmlElement not yet supported in codegen");
                return Ok(quote::quote! {
                    #opcua_path::types::Variant::Empty
                });
            }
            Variant::QualifiedName(v) => {
                let index = v.namespace_index.unwrap_or_default();
                let name = v.name.as_ref().map(|v| v.as_str()).unwrap_or("");
                quote::quote! {
                    #opcua_path::types::QualifiedName::new(#index, #name)
                }
            }
            Variant::ListOfQualifiedName(v) => {
                let mut items = quote::quote! {};
                for it in v {
                    let index = it.namespace_index.unwrap_or_default();
                    let name = it.name.as_ref().map(|v| v.as_str()).unwrap_or("");
                    items.extend(quote::quote! {
                        #opcua_path::types::QualifiedName::new(#index, #name),
                    });
                }
                quote::quote! {
                    vec![#items]
                }
            }
            Variant::LocalizedText(v) => {
                let locale = v.locale.as_ref().map(|v| v.as_str()).unwrap_or("");
                let text = v.text.as_ref().map(|v| v.as_str()).unwrap_or("");
                quote::quote! {
                    #opcua_path::types::LocalizedText::new(#locale, #text)
                }
            }
            Variant::ListOfLocalizedText(v) => {
                let mut items = quote::quote! {};
                for it in v {
                    let locale = it.locale.as_ref().map(|v| v.as_str()).unwrap_or("");
                    let text = it.text.as_ref().map(|v| v.as_str()).unwrap_or("");
                    items.extend(quote::quote! {
                        #opcua_path::types::LocalizedText::new(#locale, #text),
                    })
                }
                quote::quote! {
                    vec![#items]
                }
            }
            Variant::NodeId(v) => {
                let id = opcua_xml::schema::ua_node_set::NodeId(
                    v.identifier.clone().unwrap_or_default(),
                );
                id.render(opcua_path)?
            }
            Variant::ListOfNodeId(v) => {
                let mut items = quote::quote! {};
                for it in v {
                    let id = opcua_xml::schema::ua_node_set::NodeId(
                        it.identifier.clone().unwrap_or_default(),
                    );
                    let rendered = id.render(opcua_path)?;
                    items.extend(quote::quote! {
                        #rendered,
                    })
                }
                quote::quote! {
                    vec![#items]
                }
            }
            Variant::ExpandedNodeId(v) => {
                let id = opcua_xml::schema::ua_node_set::NodeId(
                    v.identifier.clone().unwrap_or_default(),
                );
                let r = id.render(opcua_path)?;
                quote::quote! {
                    #opcua_path::types::ExpandedNodeId::new(#r)
                }
            }
            Variant::ListOfExpandedNodeId(v) => {
                let mut items = quote::quote! {};
                for it in v {
                    let id = opcua_xml::schema::ua_node_set::NodeId(
                        it.identifier.clone().unwrap_or_default(),
                    );
                    let rendered = id.render(opcua_path)?;
                    items.extend(quote::quote! {
                        #opcua_path::types::ExpandedNodeId::new(#rendered),
                    })
                }
                quote::quote! {
                    vec![#items]
                }
            }
            Variant::ExtensionObject(v) => self.render_extension_object(v)?,
            Variant::ListOfExtensionObject(v) => {
                let mut items = quote::quote! {};
                for it in v {
                    let rendered = self.render_extension_object(it)?;
                    items.extend(quote::quote! {
                        #rendered,
                    })
                }
                quote::quote! {
                    vec![#items]
                }
            }
            Variant::Variant(v) => {
                let inner = self.render_variant(&v)?;
                quote::quote! {
                    #opcua_path::types::Variant::Variant(Box::new(#inner))
                }
            }
            Variant::ListOfVariant(v) => {
                let mut items = quote::quote! {};
                for it in v {
                    let inner = self.render_variant(&it)?;
                    items.extend(quote::quote! {
                        #opcua_path::types::Variant::Variant(Box::new(#inner))
                    });
                }
                quote::quote! {
                    vec![#items]
                }
            }
            Variant::StatusCode(v) => {
                let code = v.code;
                quote::quote! {
                    #opcua_path::types::StatusCode::from(#code)
                }
            }
            Variant::ListOfStatusCode(v) => {
                let codes = v.iter().map(|v| v.code);
                quote::quote! {
                    vec![#(#opcua_path::types::StatusCode::from(#codes)),*]
                }
            }
        };

        Ok(quote::quote! {
            #opcua_path::types::Variant::from(#inner)
        })
    }

    fn render_extension_object(&self, obj: &ExtensionObject) -> Result<TokenStream, CodeGenError> {
        let opcua_path = self.opcua_path;
        let Some(body) = &obj.body else {
            return Ok(quote::quote! {
                #opcua_path::types::ExtensionObject::null()
            });
        };

        let content = self.render_extension_object_inner(&body.data)?;

        Ok(quote! {
            #opcua_path::types::ExtensionObject::from_message(&#content)
        })
    }

    fn render_extension_object_inner(
        &self,
        data: &XmlElement,
    ) -> Result<TokenStream, CodeGenError> {
        // Rendering the body of an extension object:
        //  - First, obtain the type of the body. This is given by the name of the
        //    tag in the only item in the Body.
        //  - If the type is an enum, try to interpret the content of the tag as the
        //    enum value. There aren't actually any examples of this in the base nodeset, but looking
        //    at other standards, the value should be on the form Key_0, so pull the key out and
        //    use it on the enum type as obtained from the nodeset.
        //  - If the type is a struct, make an instance by trying to resolve each value in the type.
        //    This is recursive, so we can try to resolve even complex structures.
        //    When considering nested types we also have to handle primitives.

        // This field must have a name that matches the type name as defined in the xsd file.
        let ty = &data.tag;

        // Is the type a ListOf type? We don't support that at all in this position, since the standard
        // doesn't actually define data types for the ListOf items.
        if ty.starts_with("ListOf") {
            return Err(CodeGenError::Other(format!(
            "Got ListOf type inside extension object, this is not supported, use ListOfExtensionObject instead."
        )));
        }

        let Some(typ) = self.types.get(ty) else {
            return Err(CodeGenError::Other(format!("Unknown type {ty}")));
        };
        // First, we need to evaluate the type
        let type_ref = self
            .make_type_ref(typ)
            .map_err(|e| CodeGenError::Other(e))?;

        // Now for rendering the type itself,
        self.render_complex_type(&type_ref, &data)
    }

    fn render_complex_type(
        &self,
        ty: &TypeRef,
        node: &XmlElement,
    ) -> Result<TokenStream, CodeGenError> {
        match ty {
            TypeRef::Enum(e) => {
                let ident = Ident::new(e.name, Span::call_site());
                // An enum must have content
                let Some(val) = &node.text else {
                    return Err(CodeGenError::Other(format!(
                        "Expected value for type, got {node:?}"
                    )));
                };

                if e.variants.is_empty() {
                    // If the enum is empty, assume it is an option set and try to parse the
                    // value as a number.
                    let val = val
                        .parse::<i64>()
                        .map_err(|e| CodeGenError::ParseInt("Content".to_owned(), e))?;
                    let path = e.path;
                    Ok(quote! {
                        #path::#ident::from_bits_truncate(#val.into())
                    })
                } else {
                    // Else it should be on the form Key_0, parse it
                    let (key, _) = val.split_once("_").ok_or_else(|| {
                        CodeGenError::Other(format!(
                            "Invalid enum value: {val}, should be on the form Key_0"
                        ))
                    })?;
                    let key_ident = Ident::new(key, Span::call_site());
                    let path = e.path;
                    Ok(quote! {
                        #path::#ident::#key_ident
                    })
                }
            }
            TypeRef::Struct(e) => {
                let ident = Ident::new(e.name, Span::call_site());
                let mut fields = quote! {};
                for (name, field) in &e.fields {
                    let rendered = self.render_field(name, field, node)?;
                    let snake_name = Ident::new(&name.to_case(Case::Snake), Span::call_site());
                    fields.extend(quote! {
                        #snake_name: #rendered,
                    })
                }
                // TODO: Handle custom types here
                let path = e.path;
                Ok(quote! {
                    #path::#ident {
                        #fields
                    }
                })
            }
        }
    }

    fn render_field(
        &self,
        name: &str,
        field: &Element,
        node: &XmlElement,
    ) -> Result<TokenStream, CodeGenError> {
        let is_array = field
            .max_uccors
            .as_ref()
            .is_some_and(|m| !matches!(m, MaxOccurs::Count(1)));
        let Some(type_name) = &field.r#type else {
            return Err(CodeGenError::Other(format!(
                "Failed to render field, element {} has no type",
                name
            )));
        };
        let type_name = if let Some((_, t)) = type_name.split_once(":") {
            t
        } else {
            type_name
        };
        let is_primitive = Self::is_primitive(type_name);
        let ty = self
            .types
            .get(type_name)
            .map(|t| self.make_type_ref(t))
            .transpose()
            .map_err(|e| CodeGenError::Other(e))?;

        if is_array {
            let items: Vec<_> = node.children.iter().filter(|c| &c.tag == name).collect();
            if items.is_empty() {
                Ok(quote! {
                    None
                })
            } else {
                let mut it = quote! {};
                for item in items {
                    if is_primitive {
                        let rendered = self.render_primitive(item, type_name)?;
                        it.extend(quote! {
                            #rendered,
                        })
                    } else {
                        let Some(r) = &ty else {
                            return Err(CodeGenError::Other(format!("Type {type_name} not found")));
                        };
                        let rendered = self.render_complex_type(r, item)?;
                        it.extend(quote! {
                            #rendered,
                        })
                    }
                }

                Ok(quote! {
                    Some(vec![#it])
                })
            }
        } else {
            let item = node.first_child_with_name(name);
            let Some(item) = item else {
                return Ok(quote! {
                    Default::default()
                });
            };
            if is_primitive {
                self.render_primitive(item, type_name)
            } else {
                let Some(r) = &ty else {
                    return Err(CodeGenError::Other(format!("Type {type_name} not found")));
                };
                self.render_complex_type(r, item)
            }
        }
    }

    fn is_primitive(type_name: &str) -> bool {
        matches!(
            type_name,
            "boolean"
                | "byte"
                | "unsignedByte"
                | "short"
                | "unsignedShort"
                | "int"
                | "unsignedInt"
                | "long"
                | "unsignedLong"
                | "float"
                | "double"
                | "string"
                | "dateTime"
                | "Guid"
                | "base64Binary"
                | "QualifiedName"
                | "LocalizedText"
                | "NodeId"
                | "ExpandedNodeId"
                | "ExtensionObject"
                | "Variant"
                | "StatusCode"
        ) || type_name.starts_with("ListOf")
    }

    fn render_primitive(&self, node: &XmlElement, ty: &str) -> Result<TokenStream, CodeGenError> {
        if ty.starts_with("ListOf") {
            let field_type = match ty {
                "ListOfBoolean" => "boolean",
                "ListOfSByte" => "byte",
                "ListOfByte" => "unsignedByte",
                "ListOfInt16" => "short",
                "ListOfUInt16" => "unsignedShort",
                "ListOfInt32" => "int",
                "ListOfUInt32" => "unsignedInt",
                "ListOfInt64" => "long",
                "ListOfUInt64" => "unsignedLong",
                "ListOfFloat" => "float",
                "ListOfDouble" => "double",
                "ListOfString" => "string",
                "ListOfDateTime" => "dateTime",
                "ListOfGuid" => "Guid",
                "ListOfByteString" => "base64Binary",
                "ListOfQualifiedName" => "QualifiedName",
                "ListOfLocalizedText" => "LocalizedText",
                "ListOfNodeId" => "NodeId",
                "ListOfExpandedNodeId" => "ExpandedNodeId",
                "ListOfExtensionObject" => "ExtensionObject",
                "ListOfVariant" => "Variant",
                "ListOfStatusCode" => "StatusCode",
                _ => {
                    return Err(CodeGenError::Other(format!(
                        "ListOf type {ty} is not supported, use ListOfExtensionObject instead"
                    )))
                }
            };
            let field_name = &ty[6..];
            let mut items = quote! {};
            let mut any = false;
            for elem in node.children_with_name(field_name) {
                let rendered = self.render_primitive(elem, field_type)?;
                items.extend(quote! {
                    #rendered,
                });
                any = true;
            }
            if any {
                return Ok(quote! {
                    Some(vec![#items])
                });
            } else {
                return Ok(quote! { None });
            }
        }

        // Some simple types contain fields, and need special handling.
        let opcua_path = self.opcua_path;
        match ty {
            "Guid" => {
                if let Some(data) = node
                    .children
                    .iter()
                    .find(|f| f.tag == "String")
                    .and_then(|f| f.text.as_ref())
                {
                    let uuid = uuid::Uuid::parse_str(&data).map_err(|e| {
                        CodeGenError::Other(format!("Failed to parse uuid {data}: {e}"))
                    })?;
                    let bytes = uuid.as_bytes();
                    return Ok(quote! {
                        #opcua_path::types::Uuid::from_slice(&[#(#bytes),*]).unwrap()
                    });
                }
            }
            "QualifiedName" => {
                let index = node.child_content("NamespaceIndex");
                let name = node.child_content("Name").unwrap_or("");
                let index = if let Some(index) = index {
                    index.parse::<u16>()?
                } else {
                    0
                };
                return Ok(quote! {
                    #opcua_path::types::QualifiedName::new(#index, #name)
                });
            }
            "LocalizedText" => {
                let locale = node.child_content("Locale").unwrap_or("");
                let text = node.child_content("Text").unwrap_or("");
                return Ok(quote! {
                    #opcua_path::types::LocalizedText::new(#locale, #text)
                });
            }
            "NodeId" => {
                let id = node.child_content("Identifier");
                let id = opcua_xml::schema::ua_node_set::NodeId(
                    id.map(|m| m.to_owned()).unwrap_or_default(),
                );
                return id.render(opcua_path);
            }
            "ExpandedNodeId" => {
                let id = node.child_content("Identifier");
                let id = opcua_xml::schema::ua_node_set::NodeId(
                    id.map(|m| m.to_owned()).unwrap_or_default(),
                );
                let rendered = id.render(opcua_path)?;
                return Ok(quote! {
                    #opcua_path::types::ExpandedNodeId::new(#rendered)
                });
            }
            "StatusCode" => {
                let code = node.child_content("Code").unwrap_or("0");
                let code = code.parse::<u32>()?;
                return Ok(quote! {
                    #opcua_path::types::StatusCode::from(#code)
                });
            }
            "Variant" => {
                return Err(CodeGenError::Other(
                    "Nested variants are not currently supported".to_owned(),
                ))
            }
            "ExtensionObject" => {
                return Err(CodeGenError::Other(
                    "Nested extensionobjects are not currently supported".to_owned(),
                ))
            }
            _ => (),
        }

        let Some(data) = &node.text else {
            return Ok(quote! {
                Default::default()
            });
        };
        match ty {
            "boolean" => Ok(data.parse::<bool>()?.to_token_stream()),
            "byte" => Ok(data.parse::<i8>()?.to_token_stream()),
            "unsignedByte" => Ok(data.parse::<u8>()?.to_token_stream()),
            "short" => Ok(data.parse::<i16>()?.to_token_stream()),
            "unsignedShort" => Ok(data.parse::<u16>()?.to_token_stream()),
            "int" => Ok(data.parse::<i32>()?.to_token_stream()),
            "unsignedInt" => Ok(data.parse::<u32>()?.to_token_stream()),
            "long" => Ok(data.parse::<i64>()?.to_token_stream()),
            "unsignedLong" => Ok(data.parse::<u64>()?.to_token_stream()),
            "float" => Ok(data.parse::<f32>()?.to_token_stream()),
            "double" => Ok(data.parse::<f64>()?.to_token_stream()),
            "string" => Ok(quote! {
                #data.into()
            }),
            "dateTime" => {
                let ts = chrono::DateTime::parse_from_rfc3339(&data)
                    .map_err(|e| {
                        CodeGenError::Other(format!("Failed to parse datetime {data}: {e}"))
                    })?
                    .timestamp_micros();
                Ok(quote! {
                    #opcua_path::types::DateTimeUtc::from_timestamp_micros(#ts).unwrap()
                })
            }
            "base64Binary" => {
                let cleaned = data.replace('\n', "");
                Ok(quote! {
                    #opcua_path::types::ByteString::from_base64(#cleaned).unwrap()
                })
            }
            _ => unreachable!(),
        }
    }

    fn make_type_ref(&self, ty: &'a XsdTypeWithPath) -> Result<TypeRef<'a>, String> {
        // There are three scenarios we are willing to consider, this may be extended, but the number of
        // ways to define a type in xml is so huge that it's impractical to cover all of them.

        match &ty.ty {
            XsdFileType::Simple(s) => {
                // First, a simple type containing a restriction.
                let Some(SimpleDerivation::Restriction(r)) = &s.content else {
                    return Err(format!(
                        "Type {} is simple but does not contain a restriction",
                        s.name.as_ref().map(|v| v.as_str()).unwrap_or("")
                    ));
                };
                let mut variants = Vec::with_capacity(r.facets.len());
                for facet in r.facets.iter() {
                    if let Facet::Enumeration(e) = facet {
                        variants.push(e);
                    }
                }
                Ok(TypeRef::Enum(EnumRef {
                    name: s.name.as_ref().map(|v| v.as_str()).unwrap_or(""),
                    variants,
                    path: &ty.path,
                }))
            }
            XsdFileType::Complex(c) => {
                let Some(name) = c.name.as_ref() else {
                    return Err(format!("Type has no name"));
                };
                let (parent, sequence) = match &c.content {
                    // A complex type containing a complexcontent containing an extension is
                    // a struct that inherits fields from another struct.
                    Some(ComplexTypeContents::Complex(ComplexContent::Restriction(e))) => {
                        let (_, base_name) = e
                            .base
                            .base
                            .as_ref()
                            .ok_or_else(|| {
                                format!("Type {} is complex with a restriction, but no base", name)
                            })?
                            .split_once(":")
                            .ok_or_else(|| {
                                format!(
                                    "Type {} has a base type not on the form namespace:name",
                                    name
                                )
                            })?;
                        let base_type = self.types.get(base_name).ok_or_else(|| {
                            format!("Base type of {}, {} not found", name, base_name)
                        })?;
                        let TypeRef::Struct(base_type) = self.make_type_ref(base_type)? else {
                            return Err(format!("Base type of struct {} must be a struct", name));
                        };
                        let TypeDefParticle::Sequence(s) =
                            e.particle.as_ref().ok_or_else(|| {
                                format!("Type {} restriction does not contain a particle", name)
                            })?
                        else {
                            return Err(format!("Type is complex with a restriction that does not contain a sequence: {}", name));
                        };

                        (Some(base_type), s)
                    }
                    None => {
                        // If there's no extension, the sequence should live on the top object.
                        let TypeDefParticle::Sequence(s) = c
                            .particle
                            .as_ref()
                            .ok_or_else(|| format!("Type {} does not contain a particle", name))?
                        else {
                            return Err(format!(
                                "Type is complex but does not contain a sequence: {}",
                                name
                            ));
                        };
                        (None, s)
                    }
                    Some(_) => return Err(format!("Unsupported content type of type {}", name)),
                };

                // The sequence should be a list of elements, we only care about those.
                let mut elements = HashMap::new();
                for it in sequence.content.iter() {
                    if matches!(it, NestedParticle::Any(_)) {
                        return Err(format!(
                            "Structure contains any element, this type cannot be inferred: {}",
                            name
                        ));
                    }

                    let NestedParticle::Element(e) = it else {
                        continue;
                    };
                    let Some(name) = e.name.as_ref() else {
                        return Err(format!(
                            "Structure contains element with null name, this type is invalid: {}",
                            name
                        ));
                    };
                    elements.insert(name.as_str(), e);
                }

                if let Some(parent) = parent {
                    for (k, v) in parent.fields {
                        elements.insert(k, v);
                    }
                }

                // Sort the fields to ensure consistent ordering.
                let mut fields: Vec<_> = elements.into_iter().collect();
                fields.sort_by(|a, b| a.0.cmp(&b.0));

                Ok(TypeRef::Struct(StructRef {
                    name: &name,
                    fields,
                    path: &ty.path,
                }))
            }
        }
    }
}

struct EnumRef<'a> {
    variants: Vec<&'a FacetValue>,
    name: &'a str,
    path: &'a Path,
}

struct StructRef<'a> {
    fields: Vec<(&'a str, &'a Element)>,
    name: &'a str,
    path: &'a Path,
}

enum TypeRef<'a> {
    Enum(EnumRef<'a>),
    Struct(StructRef<'a>),
}
