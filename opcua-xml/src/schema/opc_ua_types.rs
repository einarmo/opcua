use std::collections::HashMap;

use chrono::Utc;
use roxmltree::Node;
use uuid::Uuid;

use crate::{
    ext::{
        children_of_type, children_with_name, first_child_of_type_req, first_child_with_name_opt,
        value_from_contents_opt,
    },
    XmlError, XmlLoad,
};
/// Owned XML element, simplified greatly.

#[derive(Debug)]
pub enum Variant {
    Boolean(bool),
    ListOfBoolean(Vec<bool>),
    SByte(i8),
    ListOfSByte(Vec<i8>),
    Byte(u8),
    ListOfByte(Vec<u8>),
    Int16(i16),
    ListOfInt16(Vec<i16>),
    UInt16(u16),
    ListOfUInt16(Vec<u16>),
    Int32(i32),
    ListOfInt32(Vec<i32>),
    UInt32(u32),
    ListOfUInt32(Vec<u32>),
    Int64(i64),
    ListOfInt64(Vec<i64>),
    UInt64(u64),
    ListOfUInt64(Vec<u64>),
    Float(f32),
    ListOfFloat(Vec<f32>),
    Double(f64),
    ListOfDouble(Vec<f64>),
    String(String),
    ListOfString(Vec<String>),
    DateTime(chrono::DateTime<Utc>),
    ListOfDateTime(Vec<chrono::DateTime<Utc>>),
    Guid(uuid::Uuid),
    ListOfGuid(Vec<uuid::Uuid>),
    ByteString(String),
    ListOfByteString(Vec<String>),
    XmlElement(Vec<XmlElement>),
    ListOfXmlElement(Vec<Vec<XmlElement>>),
    QualifiedName(QualifiedName),
    ListOfQualifiedName(Vec<QualifiedName>),
    LocalizedText(LocalizedText),
    ListOfLocalizedText(Vec<LocalizedText>),
    NodeId(NodeId),
    ListOfNodeId(Vec<NodeId>),
    ExpandedNodeId(NodeId),
    ListOfExpandedNodeId(Vec<NodeId>),
    ExtensionObject(ExtensionObject),
    ListOfExtensionObject(Vec<ExtensionObject>),
    Variant(Box<Variant>),
    ListOfVariant(Vec<Variant>),
    StatusCode(StatusCode),
    ListOfStatusCode(Vec<StatusCode>),
}

impl<'input> XmlLoad<'input> for Variant {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(match node.tag_name().name() {
            "Boolean" => Variant::Boolean(XmlLoad::load(node)?),
            "ListOfBoolean" => Variant::ListOfBoolean(children_with_name(node, "Boolean")?),
            "SByte" => Variant::SByte(XmlLoad::load(node)?),
            "ListOfSByte" => Variant::ListOfSByte(children_with_name(node, "SByte")?),
            "Byte" => Variant::Byte(XmlLoad::load(node)?),
            "ListOfByte" => Variant::ListOfByte(children_with_name(node, "Byte")?),
            "Int16" => Variant::Int16(XmlLoad::load(node)?),
            "ListOfInt16" => Variant::ListOfInt16(children_with_name(node, "Int16")?),
            "UInt16" => Variant::UInt16(XmlLoad::load(node)?),
            "ListOfUInt16" => Variant::ListOfUInt16(children_with_name(node, "UInt16")?),
            "Int32" => Variant::Int32(XmlLoad::load(node)?),
            "ListOfInt32" => Variant::ListOfInt32(children_with_name(node, "Int32")?),
            "UInt32" => Variant::UInt32(XmlLoad::load(node)?),
            "ListOfUInt32" => Variant::ListOfUInt32(children_with_name(node, "UInt32")?),
            "Int64" => Variant::Int64(XmlLoad::load(node)?),
            "ListOfInt64" => Variant::ListOfInt64(children_with_name(node, "Int64")?),
            "UInt64" => Variant::UInt64(XmlLoad::load(node)?),
            "ListOfUInt64" => Variant::ListOfUInt64(children_with_name(node, "UInt64")?),
            "Float" => Variant::Float(XmlLoad::load(node)?),
            "ListOfFloat" => Variant::ListOfFloat(children_with_name(node, "Float")?),
            "Double" => Variant::Double(XmlLoad::load(node)?),
            "ListOfDouble" => Variant::ListOfDouble(children_with_name(node, "Double")?),
            "String" => Variant::String(value_from_contents_opt(node)?.unwrap_or_default()),
            "ListOfString" => Variant::ListOfString(children_with_name(node, "String")?),
            "DateTime" => Variant::DateTime(XmlLoad::load(node)?),
            "ListOfDateTime" => Variant::ListOfDateTime(children_with_name(node, "DateTime")?),
            "Guid" => Variant::Guid(XmlLoad::load(node)?),
            "ListOfGuid" => Variant::ListOfGuid(children_with_name(node, "Guid")?),
            "ByteString" => Variant::ByteString(XmlLoad::load(node)?),
            "ListOfByteString" => {
                Variant::ListOfByteString(children_with_name(node, "ByteString")?)
            }
            "XmlElement" => Variant::XmlElement(children_of_type(node)?),
            "ListOfXmlElement" => Variant::ListOfXmlElement(
                node.children()
                    .filter(|n| n.tag_name().name() == "XmlElement")
                    .map(|n| children_of_type(&n))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            "QualifiedName" => Variant::QualifiedName(XmlLoad::load(node)?),
            "ListOfQualifiedName" => {
                Variant::ListOfQualifiedName(children_with_name(node, "QualifiedName")?)
            }
            "LocalizedText" => Variant::LocalizedText(XmlLoad::load(node)?),
            "ListOfLocalizedText" => {
                Variant::ListOfLocalizedText(children_with_name(node, "LocalizedText")?)
            }
            "NodeId" => Variant::NodeId(XmlLoad::load(node)?),
            "ListOfNodeId" => Variant::ListOfNodeId(children_with_name(node, "NodeId")?),
            "ExpandedNodeId" => Variant::ExpandedNodeId(XmlLoad::load(node)?),
            "ListOfExpandedNodeId" => {
                Variant::ListOfExpandedNodeId(children_with_name(node, "ExpandedNodeId")?)
            }
            "ExtensionObject" => Variant::ExtensionObject(XmlLoad::load(node)?),
            "ListOfExtensionObject" => {
                Variant::ListOfExtensionObject(children_with_name(node, "ExtensionObject")?)
            }
            "Variant" => Variant::Variant(Box::new(XmlLoad::load(node)?)),
            "ListOfVariant" => Variant::ListOfVariant(children_with_name(node, "Variant")?),
            "StatusCode" => Variant::StatusCode(XmlLoad::load(node)?),
            "ListOfStatusCode" => {
                Variant::ListOfStatusCode(children_with_name(node, "StatusCode")?)
            }
            r => return Err(XmlError::other(node, &format!("Unknown variant type: {r}"))),
        })
    }
}

impl<'input> XmlLoad<'input> for uuid::Uuid {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        let Some(content): Option<String> = first_child_with_name_opt(node, "String")? else {
            return Ok(Uuid::nil());
        };

        Uuid::try_parse(&content).map_err(|e| XmlError::parse_uuid(node, "content", e))
    }
}

#[derive(Debug)]
pub struct NodeId {
    pub identifier: Option<String>,
}

impl<'input> XmlLoad<'input> for NodeId {
    fn load(node: &roxmltree::Node<'_, 'input>) -> Result<Self, crate::XmlError> {
        Ok(Self {
            identifier: first_child_with_name_opt(node, "Identifier")?,
        })
    }
}

#[derive(Debug)]
pub struct StatusCode {
    pub code: u32,
}

impl<'input> XmlLoad<'input> for StatusCode {
    fn load(node: &roxmltree::Node<'_, 'input>) -> Result<Self, crate::XmlError> {
        Ok(Self {
            code: first_child_with_name_opt(node, "Code")?.unwrap_or(0),
        })
    }
}

#[derive(Debug)]
pub struct XmlElement {
    pub text: Option<String>,
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<XmlElement>,
}

impl<'input> XmlLoad<'input> for Option<XmlElement> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        let tag_name = node.tag_name().name();
        if tag_name.is_empty() {
            return Ok(None);
        };
        Ok(Some(XmlElement {
            text: node.text().and_then(|t| {
                let trimmed = t.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_owned())
                }
            }),
            tag: tag_name.to_owned(),
            attributes: node
                .attributes()
                .map(|a| (a.name().to_owned(), a.value().to_owned()))
                .collect(),
            children: children_of_type(node)?,
        }))
    }
}

impl XmlElement {
    pub fn children_with_name<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &XmlElement> + 'a {
        self.children.iter().filter(move |c| c.tag == name)
    }

    pub fn first_child_with_name<'a>(&'a self, name: &'a str) -> Option<&'a XmlElement> {
        self.children_with_name(name).next()
    }

    pub fn child_content<'a>(&'a self, name: &'a str) -> Option<&'a str> {
        self.first_child_with_name(name)
            .and_then(|c| c.text.as_ref())
            .map(|c| c.as_str())
    }
}

#[derive(Debug)]
pub struct QualifiedName {
    pub namespace_index: Option<u16>,
    pub name: Option<String>,
}

impl<'input> XmlLoad<'input> for QualifiedName {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            namespace_index: first_child_with_name_opt(node, "NamespaceIndex")?,
            name: first_child_with_name_opt(node, "Name")?,
        })
    }
}

#[derive(Debug)]
pub struct LocalizedText {
    pub locale: Option<String>,
    pub text: Option<String>,
}

impl<'input> XmlLoad<'input> for LocalizedText {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            locale: first_child_with_name_opt(node, "Locale")?,
            text: first_child_with_name_opt(node, "Text")?,
        })
    }
}

#[derive(Debug)]
pub struct ExtensionObjectBody {
    pub data: XmlElement,
}

impl<'input> XmlLoad<'input> for ExtensionObjectBody {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            data: first_child_of_type_req(node, "Body")?,
        })
    }
}

#[derive(Debug)]
pub struct ExtensionObject {
    pub type_id: Option<NodeId>,
    pub body: Option<ExtensionObjectBody>,
}

impl<'input> XmlLoad<'input> for ExtensionObject {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            type_id: first_child_with_name_opt(node, "TypeId")?,
            body: first_child_with_name_opt(node, "Body")?,
        })
    }
}
