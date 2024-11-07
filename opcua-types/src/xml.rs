use std::{collections::HashMap, str::FromStr, sync::Arc};

use log::warn;
pub use opcua_xml::schema::opc_ua_types::XmlElement;
use thiserror::Error;

use crate::{
    Array, ByteString, DataValue, DateTime, ExpandedNodeId, ExtensionObject, Guid, LocalizedText,
    NodeId, NodeSetNamespaceMapper, QualifiedName, StatusCode, UAString, UninitializedIndex,
    Variant, VariantScalarTypeId,
};

#[derive(Error, Debug)]
pub enum FromXmlError {
    #[error("Missing required field in XML: {0}")]
    MissingRequired(&'static str),
    #[error("Field {0} is missing required content")]
    MissingContent(&'static str),
    #[error("{0}")]
    Other(String),
    #[error("Uninitialized index {0}")]
    UninitializedIndex(u16),
}

impl From<UninitializedIndex> for FromXmlError {
    fn from(value: UninitializedIndex) -> Self {
        Self::UninitializedIndex(value.0)
    }
}

macro_rules! from_xml_number {
    ($n:ty) => {
        impl FromXml for $n {
            fn from_xml(element: &XmlElement, _ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
                let Some(c) = element.text.as_ref() else {
                    return Ok(Self::default());
                };
                Ok(c.parse::<$n>().map_err(|e| e.to_string())?)
            }
        }
    };
}

from_xml_number!(u8);
from_xml_number!(i8);
from_xml_number!(u16);
from_xml_number!(i16);
from_xml_number!(u32);
from_xml_number!(i32);
from_xml_number!(u64);
from_xml_number!(i64);
from_xml_number!(f32);
from_xml_number!(f64);
from_xml_number!(bool);

impl From<String> for FromXmlError {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

pub struct XmlContext<'a> {
    pub aliases: HashMap<String, String>,
    pub namespaces: &'a NodeSetNamespaceMapper<'a>,
    pub loaders: Vec<Arc<dyn XmlLoader>>,
}

impl<'a> XmlContext<'a> {
    fn load_extension_object(
        &self,
        body: &XmlElement,
        node_id: &NodeId,
    ) -> Result<ExtensionObject, FromXmlError> {
        for loader in &self.loaders {
            if let Some(r) = loader.load_extension_object(body, node_id, self) {
                return r;
            }
        }
        Err(FromXmlError::Other(format!(
            "No loader defined for type {node_id}"
        )))
    }
}

/// `FromXml` is implemented by types that can be loaded from a NodeSet2 XML node.
pub trait FromXml: Sized {
    /// Attempt to load the type from the given XML node.
    fn from_xml(element: &XmlElement, ctx: &XmlContext<'_>) -> Result<Self, FromXmlError>;
    /// Get the default value of the field, or fail with a `MissingRequired` error.
    /// Workaround for specialization.
    fn default_or_required(name: &'static str) -> Result<Self, FromXmlError> {
        Err(FromXmlError::MissingRequired(name))
    }
}

/// Trait for a type that can create an extension object given a node ID.
/// If the loader does not recognize the given node ID, it should return [`None`]
pub trait XmlLoader {
    /// Try to create an extension object from `body`. If `node_id` is not known,
    /// this should return [`None`], else it should return the result of calling [`FromXml::from_xml`]
    /// for the specified type.
    fn load_extension_object(
        &self,
        body: &XmlElement,
        node_id: &NodeId,
        ctx: &XmlContext<'_>,
    ) -> Option<Result<ExtensionObject, FromXmlError>>;
}

impl FromXml for UAString {
    fn from_xml(element: &XmlElement, _ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        Ok(element.text.clone().into())
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::null())
    }
}

impl FromXml for LocalizedText {
    fn from_xml(element: &XmlElement, _ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        Ok(LocalizedText::new(
            element.child_content("Locale").unwrap_or("").trim(),
            element.child_content("Text").unwrap_or("").trim(),
        ))
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::null())
    }
}

impl FromXml for Guid {
    fn from_xml(element: &XmlElement, _ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        if let Some(data) = element.child_content("String") {
            Ok(Guid::from_str(data).map_err(|_| "Failed to parse UUID".to_owned())?)
        } else {
            Ok(Guid::null())
        }
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::null())
    }
}

impl FromXml for NodeId {
    fn from_xml(element: &XmlElement, ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        let Some(id) = element.child_content("Identifier") else {
            return Ok(NodeId::null());
        };
        let id = if let Some(aliased) = ctx.aliases.get(id) {
            aliased.as_str()
        } else {
            id
        };
        let mut node_id = NodeId::from_str(id)
            .map_err(|e| format!("Failed to parse node ID from string {id}: {e}"))?;
        // Update the namespace index, the index in the XML nodeset will probably not match the one
        // in the server.
        node_id.namespace = ctx.namespaces.get_index(node_id.namespace)?;
        Ok(node_id)
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::null())
    }
}

impl FromXml for ExpandedNodeId {
    fn from_xml(element: &XmlElement, ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        Ok(ExpandedNodeId::new(NodeId::from_xml(element, ctx)?))
    }
}

impl FromXml for StatusCode {
    fn from_xml(element: &XmlElement, ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        let code = element
            .first_child_with_name("Code")
            .map(|v| u32::from_xml(v, ctx))
            .transpose()?
            .unwrap_or_default();
        Ok(StatusCode::from(code))
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::Good)
    }
}

impl FromXml for ExtensionObject {
    fn from_xml(element: &XmlElement, ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        let type_id = element
            .first_child_with_name("TypeId")
            .ok_or(FromXmlError::MissingRequired("TypeId"))?;
        let type_id = NodeId::from_xml(type_id, ctx)?;
        let body = element
            .first_child_with_name("Body")
            // Extension objects always contain the name of the type wrapping the actual type, we need to
            // unwrap that to get to the type FromXml expects.
            .and_then(|b| b.children.iter().next().and_then(|m| m.1.iter().next()));
        let Some(body) = body else {
            return Ok(ExtensionObject::null());
        };
        ctx.load_extension_object(body, &type_id)
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::null())
    }
}

impl FromXml for DateTime {
    fn from_xml(element: &XmlElement, _ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        Ok(DateTime::from_str(
            element
                .text
                .as_deref()
                .ok_or(FromXmlError::MissingContent("DateTime"))?,
        )
        .map_err(|e| e.to_string())?)
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::null())
    }
}

impl FromXml for ByteString {
    fn from_xml(element: &XmlElement, _ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        let Some(c) = element.text.as_ref() else {
            return Ok(ByteString::null());
        };
        Ok(ByteString::from_base64(c)
            .ok_or_else(|| "Failed to parse bytestring from string".to_string())?)
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::null())
    }
}

impl FromXml for QualifiedName {
    fn from_xml(element: &XmlElement, ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        let index = element.child_content("NamespaceIndex");
        let index = if let Some(index) = index {
            index.parse::<u16>().map_err(|e| e.to_string())?
        } else {
            0
        };
        let index = ctx.namespaces.get_index(index)?;
        let name = element.child_content("Name").unwrap_or("");
        Ok(QualifiedName::new(index, name))
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::null())
    }
}

impl FromXml for DataValue {
    fn from_xml(element: &XmlElement, ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        let value = XmlField::get_xml_field(element, "Value", ctx)?;
        let status = XmlField::get_xml_field(element, "StatusCode", ctx)?;
        let source_timestamp = XmlField::get_xml_field(element, "SourceTimestamp", ctx)?;
        let source_picoseconds = XmlField::get_xml_field(element, "SourcePicoseconds", ctx)?;
        let server_timestamp = XmlField::get_xml_field(element, "ServerTimestamp", ctx)?;
        let server_picoseconds = XmlField::get_xml_field(element, "ServerPicoseconds", ctx)?;
        Ok(DataValue {
            value,
            status,
            source_timestamp,
            source_picoseconds,
            server_timestamp,
            server_picoseconds,
        })
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::null())
    }
}

fn children_with_name<T: FromXml>(
    element: &XmlElement,
    ctx: &XmlContext<'_>,
    name: &str,
) -> Result<Vec<T>, FromXmlError> {
    element
        .children_with_name(name)
        .map(|n| T::from_xml(n, ctx))
        .collect()
}

impl FromXml for Variant {
    fn from_xml(element: &XmlElement, ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        let Some((_, body)) = element.children.iter().next() else {
            return Ok(Variant::Empty);
        };
        let Some(body) = body.first() else {
            return Ok(Variant::Empty);
        };
        Ok(match body.tag.as_str() {
            "Boolean" => Variant::Boolean(FromXml::from_xml(body, ctx)?),
            "ListOfBoolean" => Variant::from(children_with_name::<bool>(body, ctx, "Boolean")?),
            "SByte" => Variant::SByte(FromXml::from_xml(body, ctx)?),
            "ListOfSByte" => Variant::from(children_with_name::<i8>(body, ctx, "SByte")?),
            "Byte" => Variant::Byte(FromXml::from_xml(body, ctx)?),
            "ListOfByte" => Variant::from(children_with_name::<u8>(body, ctx, "Byte")?),
            "Int16" => Variant::Int16(FromXml::from_xml(body, ctx)?),
            "ListOfInt16" => Variant::from(children_with_name::<i16>(body, ctx, "Int16")?),
            "UInt16" => Variant::UInt16(FromXml::from_xml(body, ctx)?),
            "ListOfUInt16" => Variant::from(children_with_name::<u16>(body, ctx, "UInt16")?),
            "Int32" => Variant::Int32(FromXml::from_xml(body, ctx)?),
            "ListOfInt32" => Variant::from(children_with_name::<i32>(body, ctx, "Int32")?),
            "UInt32" => Variant::UInt32(FromXml::from_xml(body, ctx)?),
            "ListOfUInt32" => Variant::from(children_with_name::<u32>(body, ctx, "UInt32")?),
            "Int64" => Variant::Int64(FromXml::from_xml(body, ctx)?),
            "ListOfInt64" => Variant::from(children_with_name::<i64>(body, ctx, "Int64")?),
            "UInt64" => Variant::UInt64(FromXml::from_xml(body, ctx)?),
            "ListOfUInt64" => Variant::from(children_with_name::<u64>(body, ctx, "UInt64")?),
            "Float" => Variant::Float(FromXml::from_xml(body, ctx)?),
            "ListOfFloat" => Variant::from(children_with_name::<f32>(body, ctx, "Float")?),
            "Double" => Variant::Double(FromXml::from_xml(body, ctx)?),
            "ListOfDouble" => Variant::from(children_with_name::<f64>(body, ctx, "Double")?),
            "String" => Variant::String(FromXml::from_xml(body, ctx)?),
            "ListOfString" => Variant::from(children_with_name::<UAString>(body, ctx, "String")?),
            "DateTime" => Variant::DateTime(FromXml::from_xml(body, ctx)?),
            "ListOfDateTime" => {
                Variant::from(children_with_name::<DateTime>(body, ctx, "DateTime")?)
            }
            "Guid" => Variant::Guid(FromXml::from_xml(body, ctx)?),
            "ListOfGuid" => Variant::from(children_with_name::<Guid>(body, ctx, "Guid")?),
            "ByteString" => Variant::ByteString(FromXml::from_xml(body, ctx)?),
            "ListOfByteString" => {
                Variant::from(children_with_name::<ByteString>(body, ctx, "ByteString")?)
            }
            "XmlElement" => Variant::XmlElement(body.to_string().into()),
            "ListOfXmlElement" => Variant::from(
                body.children_with_name("XmlElement")
                    .map(|v| UAString::from(v.to_string()))
                    .collect::<Vec<_>>(),
            ),
            "QualifiedName" => Variant::QualifiedName(FromXml::from_xml(body, ctx)?),
            "ListOfQualifiedName" => Variant::from(children_with_name::<QualifiedName>(
                body,
                ctx,
                "QualifiedName",
            )?),
            "LocalizedText" => Variant::LocalizedText(FromXml::from_xml(body, ctx)?),
            "ListOfLocalizedText" => Variant::from(children_with_name::<LocalizedText>(
                body,
                ctx,
                "LocalizedText",
            )?),
            "NodeId" => Variant::NodeId(FromXml::from_xml(body, ctx)?),
            "ListOfNodeId" => Variant::from(children_with_name::<NodeId>(body, ctx, "NodeId")?),
            "ExpandedNodeId" => Variant::ExpandedNodeId(FromXml::from_xml(body, ctx)?),
            "ListOfExpandedNodeId" => Variant::from(children_with_name::<ExpandedNodeId>(
                body,
                ctx,
                "ExpandedNodeId",
            )?),
            "ExtensionObject" => Variant::ExtensionObject(FromXml::from_xml(body, ctx)?),
            "ListOfExtensionObject" => Variant::from(children_with_name::<ExtensionObject>(
                body,
                ctx,
                "ExtensionObject",
            )?),
            "Variant" => Variant::Variant(FromXml::from_xml(body, ctx)?),
            "ListOfVariant" => Variant::from(children_with_name::<Variant>(body, ctx, "Variant")?),
            "StatusCode" => Variant::StatusCode(FromXml::from_xml(body, ctx)?),
            "ListOfStatusCode" => {
                Variant::from(children_with_name::<StatusCode>(body, ctx, "StatusCode")?)
            }
            r => return Err(FromXmlError::Other(format!("Unexpected variant type: {r}"))),
        })
    }

    fn default_or_required(_name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Self::Empty)
    }
}

impl<T> FromXml for Box<T>
where
    T: FromXml,
{
    fn from_xml(element: &XmlElement, ctx: &XmlContext<'_>) -> Result<Self, FromXmlError> {
        Ok(Box::new(T::from_xml(element, ctx)?))
    }

    fn default_or_required(name: &'static str) -> Result<Self, FromXmlError> {
        Ok(Box::new(T::default_or_required(name)?))
    }
}

/// `XmlField` is a convenience trait that wraps [`FromXml`] when the
/// XML node to extract is one or more fields of a parent node.
/// It is implemented for `T`, `Vec<T>`, `Option<T>`, and `Option<Vec<T>>`, notably.
pub trait XmlField: Sized {
    fn get_xml_field(
        parent: &XmlElement,
        name: &'static str,
        ctx: &XmlContext<'_>,
    ) -> Result<Self, FromXmlError>;
}

impl<T> XmlField for T
where
    T: FromXml,
{
    fn get_xml_field(
        parent: &XmlElement,
        name: &'static str,
        ctx: &XmlContext<'_>,
    ) -> Result<Self, FromXmlError> {
        let Some(own) = parent.first_child_with_name(name) else {
            return T::default_or_required(name);
        };
        FromXml::from_xml(own, ctx)
    }
}

impl<T> XmlField for Option<T>
where
    T: FromXml,
{
    fn get_xml_field(
        parent: &XmlElement,
        name: &'static str,
        ctx: &XmlContext<'_>,
    ) -> Result<Self, FromXmlError> {
        let Some(own) = parent.first_child_with_name(name) else {
            return Ok(None);
        };
        Ok(Some(FromXml::from_xml(own, ctx)?))
    }
}

impl<T> XmlField for Vec<T>
where
    T: FromXml,
{
    fn get_xml_field(
        parent: &XmlElement,
        name: &'static str,
        ctx: &XmlContext<'_>,
    ) -> Result<Self, FromXmlError> {
        parent
            .children_with_name(name)
            .map(|n| FromXml::from_xml(n, ctx))
            .collect()
    }
}

impl<T> XmlField for Option<Vec<T>>
where
    T: FromXml,
{
    fn get_xml_field(
        parent: &XmlElement,
        name: &'static str,
        ctx: &XmlContext<'_>,
    ) -> Result<Self, FromXmlError> {
        let v: Vec<T> = parent
            .children_with_name(name)
            .map(|n| <T as FromXml>::from_xml(n, ctx))
            .collect::<Result<Vec<_>, _>>()?;
        if v.is_empty() {
            Ok(None)
        } else {
            Ok(Some(v))
        }
    }
}

fn mk_node_id(
    node_id: &opc_ua_types::NodeId,
    ctx: &XmlContext<'_>,
) -> Result<NodeId, FromXmlError> {
    let Some(idf) = &node_id.identifier else {
        return Ok(NodeId::null());
    };
    let Ok(mut parsed) = NodeId::from_str(idf) else {
        warn!("Invalid node ID: {idf}");
        return Err(FromXmlError::Other(format!("Invalid node ID: {idf}")));
    };
    parsed.namespace = ctx.namespaces.get_index(parsed.namespace)?;
    Ok(parsed)
}

fn mk_extension_object(
    ext_obj: &opc_ua_types::ExtensionObject,
    ctx: &XmlContext<'_>,
) -> Result<ExtensionObject, FromXmlError> {
    let Some(b) = ext_obj.body.as_ref() else {
        return Ok(ExtensionObject::null());
    };

    let Some(type_id) = ext_obj.type_id.as_ref() else {
        return Ok(ExtensionObject::null());
    };

    let node_id = mk_node_id(type_id, ctx)?;

    ctx.load_extension_object(&b.data, &node_id)
}

use opcua_xml::schema::opc_ua_types::{self, Variant as XmlVariant};

impl Variant {
    /// Create a Variant value from a NodeSet2 variant object.
    /// Note that this is different from the `FromXml` implementation of `Variant`,
    /// which accepts an untyped XML node.
    pub fn from_nodeset(val: &XmlVariant, ctx: &XmlContext<'_>) -> Result<Variant, FromXmlError> {
        Ok(match val {
            XmlVariant::Boolean(v) => (*v).into(),
            XmlVariant::ListOfBoolean(v) => v.into(),
            XmlVariant::SByte(v) => (*v).into(),
            XmlVariant::ListOfSByte(v) => v.into(),
            XmlVariant::Byte(v) => (*v).into(),
            XmlVariant::ListOfByte(v) => v.into(),
            XmlVariant::Int16(v) => (*v).into(),
            XmlVariant::ListOfInt16(v) => v.into(),
            XmlVariant::UInt16(v) => (*v).into(),
            XmlVariant::ListOfUInt16(v) => v.into(),
            XmlVariant::Int32(v) => (*v).into(),
            XmlVariant::ListOfInt32(v) => v.into(),
            XmlVariant::UInt32(v) => (*v).into(),
            XmlVariant::ListOfUInt32(v) => v.into(),
            XmlVariant::Int64(v) => (*v).into(),
            XmlVariant::ListOfInt64(v) => v.into(),
            XmlVariant::UInt64(v) => (*v).into(),
            XmlVariant::ListOfUInt64(v) => v.into(),
            XmlVariant::Float(v) => (*v).into(),
            XmlVariant::ListOfFloat(v) => v.into(),
            XmlVariant::Double(v) => (*v).into(),
            XmlVariant::ListOfDouble(v) => v.into(),
            XmlVariant::String(v) => v.clone().into(),
            XmlVariant::ListOfString(v) => v.into(),
            XmlVariant::DateTime(v) => (*v).into(),
            XmlVariant::ListOfDateTime(v) => v.into(),
            XmlVariant::Guid(v) => (*v).into(),
            XmlVariant::ListOfGuid(v) => v.into(),
            XmlVariant::ByteString(b) => ByteString::from_base64(b.trim())
                .unwrap_or_else(|| {
                    warn!("Invalid byte string: {b}");
                    ByteString::null()
                })
                .into(),
            XmlVariant::ListOfByteString(v) => v
                .iter()
                .map(|b| {
                    ByteString::from_base64(b.trim()).unwrap_or_else(|| {
                        warn!("Invalid byte string: {b}");
                        ByteString::null()
                    })
                })
                .collect::<Vec<_>>()
                .into(),
            XmlVariant::XmlElement(vec) => Variant::XmlElement(
                vec.iter()
                    .map(|v| v.to_string().trim().to_owned())
                    .collect::<String>()
                    .into(),
            ),
            XmlVariant::ListOfXmlElement(vec) => Variant::Array(Box::new(Array {
                value_type: VariantScalarTypeId::XmlElement,
                values: vec
                    .iter()
                    .map(|v| {
                        Variant::XmlElement(
                            v.iter()
                                .map(|vv| vv.to_string().trim().to_string())
                                .collect::<String>()
                                .into(),
                        )
                    })
                    .collect(),
                dimensions: None,
            })),
            XmlVariant::QualifiedName(q) => QualifiedName::new(
                ctx.namespaces.get_index(q.namespace_index.unwrap_or(0))?,
                q.name.as_deref().unwrap_or("").trim(),
            )
            .into(),
            XmlVariant::ListOfQualifiedName(v) => v
                .iter()
                .map(|q| {
                    Ok(QualifiedName::new(
                        ctx.namespaces.get_index(q.namespace_index.unwrap_or(0))?,
                        q.name.as_deref().unwrap_or("").trim(),
                    ))
                })
                .collect::<Result<Vec<QualifiedName>, FromXmlError>>()?
                .into(),
            XmlVariant::LocalizedText(l) => LocalizedText::new(
                l.locale.as_deref().unwrap_or("").trim(),
                l.text.as_deref().unwrap_or("").trim(),
            )
            .into(),
            XmlVariant::ListOfLocalizedText(v) => v
                .iter()
                .map(|l| {
                    LocalizedText::new(
                        l.locale.as_deref().unwrap_or("").trim(),
                        l.text.as_deref().unwrap_or("").trim(),
                    )
                })
                .collect::<Vec<_>>()
                .into(),
            XmlVariant::NodeId(node_id) => mk_node_id(node_id, ctx)?.into(),
            XmlVariant::ListOfNodeId(v) => v
                .iter()
                .map(|node_id| mk_node_id(node_id, ctx))
                .collect::<Result<Vec<_>, _>>()?
                .into(),
            XmlVariant::ExpandedNodeId(node_id) => {
                ExpandedNodeId::new(mk_node_id(node_id, ctx)?).into()
            }
            XmlVariant::ListOfExpandedNodeId(v) => v
                .iter()
                .map(|node_id| Ok(ExpandedNodeId::new(mk_node_id(node_id, ctx)?)))
                .collect::<Result<Vec<_>, FromXmlError>>()?
                .into(),
            XmlVariant::ExtensionObject(val) => mk_extension_object(val, ctx)?.into(),
            XmlVariant::ListOfExtensionObject(v) => v
                .iter()
                .map(|val| mk_extension_object(val, ctx))
                .collect::<Result<Vec<_>, _>>()?
                .into(),
            XmlVariant::Variant(variant) => {
                let inner = Variant::from_nodeset(variant, ctx)?;
                Variant::Variant(Box::new(inner))
            }
            XmlVariant::ListOfVariant(vec) => Variant::Array(Box::new(Array {
                value_type: VariantScalarTypeId::Variant,
                values: vec
                    .iter()
                    .map(|v| Ok(Variant::Variant(Box::new(Variant::from_nodeset(v, ctx)?))))
                    .collect::<Result<Vec<_>, FromXmlError>>()?,
                dimensions: None,
            })),
            XmlVariant::StatusCode(status_code) => StatusCode::from(status_code.code).into(),
            XmlVariant::ListOfStatusCode(vec) => vec
                .iter()
                .map(|v| StatusCode::from(v.code))
                .collect::<Vec<_>>()
                .into(),
        })
    }
}
