//! This module contains an implementation of the OPCBinarySchema.xsd XML schema,
//! for use with code generation.
//! Attributes such as `any` or `anyAttribute` are not added.

use roxmltree::{Document, Node};

use crate::{error::XmlError, ext::NodeExt, XmlLoad};

#[derive(Debug)]
pub struct Documentation {
    pub contents: Option<String>,
}

impl<'input> XmlLoad<'input> for Documentation {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            contents: node.text().map(|a| a.to_owned()),
        })
    }
}

#[derive(Debug)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

#[derive(Debug)]
pub struct TypeDescription {
    pub documentation: Option<Documentation>,
    pub name: String,
    pub default_byte_order: Option<ByteOrder>,
}

impl<'input> XmlLoad<'input> for TypeDescription {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            documentation: node
                .first_child_with_name("Documentation")
                .ok()
                .and_then(|d| Documentation::load(&d).ok()),
            name: node.try_attribute("Name")?.to_owned(),
            default_byte_order: match node.attribute("DefaultByteOrder") {
                Some("LittleEndian") => Some(ByteOrder::LittleEndian),
                Some("BigEndian") => Some(ByteOrder::BigEndian),
                Some(r) => {
                    return Err(XmlError::other(
                        &node,
                        &format!(
                            "Expected LittleEndian or BigEndian for DefaultByteOrder, got {r}"
                        ),
                    ))
                }
                None => None,
            },
        })
    }
}

#[derive(Debug)]
pub struct OpaqueType {
    pub description: TypeDescription,
    pub length_in_bits: Option<i64>,
    pub byte_order_significant: bool,
}

impl<'input> XmlLoad<'input> for OpaqueType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            description: TypeDescription::load(node)?,
            length_in_bits: node
                .attribute("LengthInBits")
                .map(|v| {
                    v.parse()
                        .map_err(|e| XmlError::parse_int(node, "LengthInBits", e))
                })
                .transpose()?,
            byte_order_significant: node.attribute("ByteOrderSignificant") == Some("true"),
        })
    }
}

#[derive(Debug)]
pub struct EnumeratedValue {
    pub documentation: Option<Documentation>,
    pub name: Option<String>,
    pub value: Option<i64>,
}
impl<'input> XmlLoad<'input> for EnumeratedValue {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            documentation: node
                .first_child_with_name("Documentation")
                .ok()
                .and_then(|d| Documentation::load(&d).ok()),
            name: node.attribute("Name").map(|n| n.to_owned()),
            value: node
                .attribute("Value")
                .map(|v| v.parse().map_err(|e| XmlError::parse_int(node, "Value", e)))
                .transpose()?,
        })
    }
}

#[derive(Debug)]
pub struct EnumeratedType {
    pub opaque: OpaqueType,
    pub variants: Vec<EnumeratedValue>,
    pub is_option_set: bool,
}

impl<'input> XmlLoad<'input> for EnumeratedType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            opaque: OpaqueType::load(node)?,
            variants: node
                .with_name("EnumeratedValue")
                .map(|v| EnumeratedValue::load(&v))
                .collect::<Result<Vec<_>, _>>()?,
            is_option_set: node.attribute("IsOptionSet") == Some("true"),
        })
    }
}

#[derive(Debug)]
pub enum SwitchOperand {
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqual,
}

#[derive(Debug)]
pub struct FieldType {
    pub documentation: Option<Documentation>,
    pub name: String,
    pub type_name: Option<String>,
    pub length: Option<u64>,
    pub length_field: Option<String>,
    pub is_length_in_bytes: bool,
    pub switch_field: Option<String>,
    pub switch_value: Option<u64>,
    pub switch_operand: Option<SwitchOperand>,
    pub terminator: Option<String>,
}

impl<'input> XmlLoad<'input> for FieldType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            documentation: node
                .first_child_with_name("Documentation")
                .ok()
                .and_then(|d| Documentation::load(&d).ok()),
            name: node.try_attribute("Name")?.to_owned(),
            type_name: node.attribute("TypeName").map(|a| a.to_owned()),
            length: node
                .attribute("Length")
                .map(|a| a.parse())
                .transpose()
                .map_err(|e| XmlError::parse_int(node, "Length", e))?,
            length_field: node.attribute("LengthField").map(|a| a.to_owned()),
            is_length_in_bytes: node.attribute("IsLengthInBytes") == Some("true"),
            switch_field: node.attribute("SwitchField").map(|a| a.to_owned()),
            switch_value: node
                .attribute("SwitchValue")
                .map(|a| a.parse())
                .transpose()
                .map_err(|e| XmlError::parse_int(node, "SwitchValue", e))?,
            switch_operand: match node.attribute("SwitchOperand") {
                Some("Equals") => Some(SwitchOperand::Equals),
                Some("GreaterThan") => Some(SwitchOperand::GreaterThan),
                Some("LessThan") => Some(SwitchOperand::LessThan),
                Some("GreaterThanOrEqual") => Some(SwitchOperand::GreaterThanOrEqual),
                Some("LessThanOrEqual") => Some(SwitchOperand::LessThanOrEqual),
                Some("NotEqual") => Some(SwitchOperand::NotEqual),
                Some(r) => {
                    return Err(XmlError::other(
                        node,
                        &format!("Unexpected value for SwitchOperand: {r}"),
                    ))
                }
                _ => None,
            },
            terminator: node.attribute("Terminator").map(|a| a.to_owned()),
        })
    }
}

#[derive(Debug)]
pub struct StructuredType {
    pub description: TypeDescription,
    pub fields: Vec<FieldType>,
    pub base_type: Option<String>,
}

impl<'input> XmlLoad<'input> for StructuredType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            description: TypeDescription::load(node)?,
            fields: node
                .with_name("Field")
                .map(|e| FieldType::load(&e))
                .collect::<Result<Vec<_>, _>>()?,
            base_type: node.attribute("BaseType").map(|t| t.to_owned()),
        })
    }
}

#[derive(Debug)]
pub struct ImportDirective {
    pub namespace: Option<String>,
    pub location: Option<String>,
}

impl<'input> XmlLoad<'input> for ImportDirective {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            namespace: node.attribute("Namespace").map(|a| a.to_owned()),
            location: node.attribute("Location").map(|a| a.to_owned()),
        })
    }
}

#[derive(Debug)]
pub enum TypeDictionaryItem {
    Opaque(OpaqueType),
    Enumerated(EnumeratedType),
    Structured(StructuredType),
}

#[derive(Debug)]
pub struct TypeDictionary {
    pub documentation: Option<Documentation>,
    pub imports: Vec<ImportDirective>,
    pub elements: Vec<TypeDictionaryItem>,
    pub target_namespace: String,
    pub default_byte_order: Option<ByteOrder>,
}

impl<'input> XmlLoad<'input> for TypeDictionary {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            documentation: node
                .first_child_with_name("Documentation")
                .ok()
                .and_then(|d| Documentation::load(&d).ok()),
            imports: node
                .with_name("Import")
                .map(|e| ImportDirective::load(&e))
                .collect::<Result<Vec<_>, _>>()?,
            elements: node
                .children()
                .filter_map(|e| match e.tag_name().name() {
                    "OpaqueType" => Some(OpaqueType::load(&e).map(TypeDictionaryItem::Opaque)),
                    "EnumeratedType" => {
                        Some(EnumeratedType::load(&e).map(TypeDictionaryItem::Enumerated))
                    }
                    "StructuredType" => {
                        Some(StructuredType::load(&e).map(TypeDictionaryItem::Structured))
                    }
                    _ => None,
                })
                .collect::<Result<Vec<_>, _>>()?,
            target_namespace: node.try_attribute("TargetNamespace")?.to_owned(),
            default_byte_order: match node.attribute("DefaultByteOrder") {
                Some("LittleEndian") => Some(ByteOrder::LittleEndian),
                Some("BigEndian") => Some(ByteOrder::BigEndian),
                Some(r) => {
                    return Err(XmlError::other(
                        &node,
                        &format!(
                            "Expected LittleEndian or BigEndian for DefaultByteOrder, got {r}"
                        ),
                    ))
                }
                None => None,
            },
        })
    }
}

pub fn load_bsd_file(document: &str) -> Result<TypeDictionary, XmlError> {
    let document = Document::parse(document).map_err(|e| XmlError {
        span: 0..1,
        error: crate::error::XmlErrorInner::XML(e),
    })?;
    TypeDictionary::load(&document.root().first_child_with_name("TypeDictionary")?)
}
