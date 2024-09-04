//! A limited implementation of code generation based on an xml schema. Adapted for
//! OPC-UA code generation.

use roxmltree::{Document, Node};

use crate::{
    ext::{
        children_of_type, children_with_name, first_child_of_type, first_child_of_type_req,
        first_child_with_name, first_child_with_name_opt, value_from_attr, value_from_attr_opt,
    },
    FromValue, XmlError, XmlLoad,
};

pub fn load_xsd_schema(document: &str) -> Result<XmlSchema, XmlError> {
    let document = Document::parse(document).map_err(|e| XmlError {
        span: 0..1,
        error: crate::error::XmlErrorInner::Xml(e),
    })?;
    let root = document.root();
    first_child_with_name(&root, "schema")
}

#[derive(Debug)]
pub struct FacetValue {
    pub value: String,
    pub fixed: bool,
}

impl<'input> XmlLoad<'input> for FacetValue {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            value: value_from_attr(node, "value")?,
            fixed: value_from_attr_opt(node, "fixed")?.unwrap_or(false),
        })
    }
}

#[derive(Debug)]
pub enum Facet {
    MinExclusive(FacetValue),
    MinInclusive(FacetValue),
    MaxExclusive(FacetValue),
    MaxInclusive(FacetValue),
    TotalDigits(FacetValue),
    FractionDigits(FacetValue),
    Length(FacetValue),
    MinLength(FacetValue),
    MaxLength(FacetValue),
    Enumeration(FacetValue),
    WhiteSpace(FacetValue),
    Pattern(FacetValue),
}

impl<'input> XmlLoad<'input> for Option<Facet> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "minExclusive" => Facet::MinExclusive(FacetValue::load(node)?),
            "minInclusive" => Facet::MinInclusive(FacetValue::load(node)?),
            "maxExclusive" => Facet::MaxExclusive(FacetValue::load(node)?),
            "maxInclusive" => Facet::MaxInclusive(FacetValue::load(node)?),
            "totalDigits" => Facet::TotalDigits(FacetValue::load(node)?),
            "fractionDigits" => Facet::FractionDigits(FacetValue::load(node)?),
            "length" => Facet::Length(FacetValue::load(node)?),
            "minLength" => Facet::MinLength(FacetValue::load(node)?),
            "maxLength" => Facet::MaxLength(FacetValue::load(node)?),
            "enumeration" => Facet::Enumeration(FacetValue::load(node)?),
            "whiteSpace" => Facet::WhiteSpace(FacetValue::load(node)?),
            "pattern" => Facet::Pattern(FacetValue::load(node)?),
            _ => return Ok(None),
        }))
    }
}

#[derive(Debug)]
pub struct Restriction {
    pub base: Option<String>,
    pub facets: Vec<Facet>,
    pub content: Option<SimpleType>,
    pub particle: Option<TypeDefParticle>,
    pub attributes: Vec<Attribute>,
}

impl<'input> XmlLoad<'input> for Restriction {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: value_from_attr_opt(node, "base")?,
            facets: children_of_type(node)?,
            particle: first_child_of_type(node)?,
            attributes: children_with_name(node, "")?,
            content: first_child_with_name_opt(node, "simpleType")?,
        })
    }
}

#[derive(Debug)]
pub struct List {
    pub content: Option<SimpleType>,
    pub item_type: Option<String>,
}

impl<'input> XmlLoad<'input> for List {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            content: first_child_with_name_opt(node, "simpleType")?,
            item_type: value_from_attr_opt(node, "itemType")?,
        })
    }
}

#[derive(Debug)]
pub struct Union {
    pub variants: Vec<SimpleType>,
    pub member_types: Option<Vec<String>>,
}

impl<'input> XmlLoad<'input> for Union {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            variants: children_with_name(node, "simpleType")?,
            member_types: value_from_attr_opt(node, "memberTypes")?,
        })
    }
}

#[derive(Debug)]
pub enum SimpleDerivation {
    Restriction(Box<Restriction>),
    List(Box<List>),
    Union(Box<Union>),
}

impl<'input> XmlLoad<'input> for Option<SimpleDerivation> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "restriction" => SimpleDerivation::Restriction(Box::new(XmlLoad::load(node)?)),
            "list" => SimpleDerivation::List(Box::new(XmlLoad::load(node)?)),
            "union" => SimpleDerivation::Union(Box::new(XmlLoad::load(node)?)),
            _ => return Ok(None),
        }))
    }
}

#[derive(Debug)]
pub struct SimpleType {
    pub name: Option<String>,
    pub content: Option<SimpleDerivation>,
}

impl<'input> XmlLoad<'input> for SimpleType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            name: value_from_attr_opt(node, "name")?,
            content: first_child_of_type(node)?,
        })
    }
}

#[derive(Debug)]
pub struct Any {
    pub min_occurs: Option<u32>,
    pub max_uccors: Option<MaxOccurs>,
}

impl<'input> XmlLoad<'input> for Any {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            min_occurs: value_from_attr_opt(node, "minOccurs")?,
            max_uccors: value_from_attr_opt(node, "maxOccurs")?,
        })
    }
}

pub enum Particle {
    Element(Element),
    All(Group),
    Choice(Group),
    Sequence(Group),
    Any(Any),
}

impl<'input> XmlLoad<'input> for Option<Particle> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "element" => Particle::Element(XmlLoad::load(node)?),
            "all" => Particle::All(XmlLoad::load(node)?),
            "choice" => Particle::Choice(XmlLoad::load(node)?),
            "sequence" => Particle::Sequence(XmlLoad::load(node)?),
            "any" => Particle::Any(XmlLoad::load(node)?),
            _ => return Ok(None),
        }))
    }
}

#[derive(Debug)]
pub enum NestedParticle {
    Element(Element),
    Choice(Group),
    Sequence(Group),
    Any(Any),
}

impl<'input> XmlLoad<'input> for Option<NestedParticle> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "element" => NestedParticle::Element(XmlLoad::load(node)?),
            "choice" => NestedParticle::Choice(XmlLoad::load(node)?),
            "sequence" => NestedParticle::Sequence(XmlLoad::load(node)?),
            "any" => NestedParticle::Any(XmlLoad::load(node)?),
            _ => return Ok(None),
        }))
    }
}

#[derive(Debug)]
pub struct Group {
    pub content: Vec<NestedParticle>,
    pub min_occurs: Option<u32>,
    pub max_uccors: Option<MaxOccurs>,
}

impl<'input> XmlLoad<'input> for Group {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            content: children_of_type(node)?,
            min_occurs: value_from_attr_opt(node, "minOccurs")?,
            max_uccors: value_from_attr_opt(node, "maxOccurs")?,
        })
    }
}

#[derive(Debug)]
pub enum TypeDefParticle {
    All(Group),
    Choice(Group),
    Sequence(Group),
}

impl<'input> XmlLoad<'input> for Option<TypeDefParticle> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "all" => TypeDefParticle::All(XmlLoad::load(node)?),
            "choice" => TypeDefParticle::Choice(XmlLoad::load(node)?),
            "sequence" => TypeDefParticle::Sequence(XmlLoad::load(node)?),
            _ => return Ok(None),
        }))
    }
}

#[derive(Debug)]
pub struct Extension {
    pub content: Option<TypeDefParticle>,
    pub attributes: Vec<Attribute>,
    pub base: String,
}

impl<'input> XmlLoad<'input> for Extension {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            content: first_child_of_type(node)?,
            attributes: children_with_name(node, "attributes")?,
            base: value_from_attr(node, "base")?,
        })
    }
}

#[derive(Debug)]
pub enum SimpleContent {
    Restriction(Restriction),
    Extension(Extension),
}

impl<'input> XmlLoad<'input> for Option<SimpleContent> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "restriction" => SimpleContent::Restriction(XmlLoad::load(node)?),
            "extension" => SimpleContent::Extension(XmlLoad::load(node)?),
            _ => return Ok(None),
        }))
    }
}

#[derive(Debug)]
pub struct ComplexRestriction {
    pub base: Restriction,
    pub particle: Option<TypeDefParticle>,
}

impl<'input> XmlLoad<'input> for ComplexRestriction {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            base: Restriction::load(node)?,
            particle: first_child_of_type(node)?,
        })
    }
}

#[derive(Debug)]
pub enum ComplexContent {
    Restriction(ComplexRestriction),
    Extension(Extension),
}

impl<'input> XmlLoad<'input> for Option<ComplexContent> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "restriction" => ComplexContent::Restriction(XmlLoad::load(node)?),
            "extension" => ComplexContent::Extension(XmlLoad::load(node)?),
            _ => return Ok(None),
        }))
    }
}

#[derive(Debug)]
pub enum ComplexTypeContents {
    Simple(SimpleContent),
    Complex(ComplexContent),
}

impl<'input> XmlLoad<'input> for Option<ComplexTypeContents> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "simpleContent" => ComplexTypeContents::Simple(first_child_of_type_req(
                node,
                "restriction or extension",
            )?),
            "complexContent" => ComplexTypeContents::Complex(first_child_of_type_req(
                node,
                "restriction or extension",
            )?),
            _ => {
                return Ok(None);
            }
        }))
    }
}

#[derive(Debug)]
pub struct ComplexType {
    pub content: Option<ComplexTypeContents>,
    pub particle: Option<TypeDefParticle>,
    pub attributes: Vec<Attribute>,
    pub name: Option<String>,
}

impl<'input> XmlLoad<'input> for ComplexType {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            content: first_child_of_type(node)?,
            particle: first_child_of_type(node)?,
            attributes: children_with_name(node, "attribute")?,
            name: value_from_attr_opt(node, "name")?,
        })
    }
}

#[derive(Debug)]
pub enum ElementContents {
    SimpleType(SimpleType),
    ComplexType(ComplexType),
}

impl<'input> XmlLoad<'input> for Option<ElementContents> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "simpleType" => ElementContents::SimpleType(XmlLoad::load(node)?),
            "complexType" => ElementContents::ComplexType(XmlLoad::load(node)?),
            _ => return Ok(None),
        }))
    }
}

#[derive(Debug)]
pub enum MaxOccurs {
    Count(u32),
    Unbounded,
}

impl FromValue for MaxOccurs {
    fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError> {
        if v == "unbounded" {
            return Ok(Self::Unbounded);
        }

        Ok(Self::Count(u32::from_value(node, attr, v)?))
    }
}

#[derive(Debug)]
pub enum AttributeUse {
    Prohibited,
    Optional,
    Required,
}

impl FromValue for AttributeUse {
    fn from_value(node: &Node<'_, '_>, attr: &str, v: &str) -> Result<Self, XmlError> {
        match v {
            "prohibited" => Ok(Self::Prohibited),
            "optional" => Ok(Self::Optional),
            "required" => Ok(Self::Required),
            r => Err(XmlError::other(
                node,
                &format!("Unexpected value for {attr}: {r}"),
            )),
        }
    }
}

#[derive(Debug)]
pub struct Attribute {
    pub content: Option<SimpleType>,
    pub name: Option<String>,
    pub r#ref: Option<String>,
    pub r#type: Option<String>,
    pub r#use: AttributeUse,
    pub default: Option<String>,
}

impl<'input> XmlLoad<'input> for Attribute {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            content: first_child_with_name_opt(node, "simpleType")?,
            name: value_from_attr_opt(node, "name")?,
            r#ref: value_from_attr_opt(node, "ref")?,
            r#type: value_from_attr_opt(node, "type")?,
            r#use: value_from_attr_opt(node, "use")?.unwrap_or(AttributeUse::Optional),
            default: value_from_attr_opt(node, "default")?,
        })
    }
}

#[derive(Debug)]
pub struct AttrDecls {
    pub attributes: Vec<Attribute>,
}

impl<'input> XmlLoad<'input> for AttrDecls {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            attributes: children_with_name(node, "attribute")?,
        })
    }
}

#[derive(Debug)]
pub struct Element {
    pub r#type: Option<String>,
    pub default: Option<String>,
    pub nillable: bool,
    pub contents: Option<ElementContents>,
    pub name: Option<String>,
    pub r#ref: Option<String>,
    pub min_occurs: Option<u32>,
    pub max_uccors: Option<MaxOccurs>,
}

impl<'input> XmlLoad<'input> for Element {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            r#type: value_from_attr_opt(node, "type")?,
            default: value_from_attr_opt(node, "default")?,
            nillable: value_from_attr_opt(node, "nillable")?.unwrap_or(false),
            name: value_from_attr_opt(node, "name")?,
            r#ref: value_from_attr_opt(node, "ref")?,
            min_occurs: value_from_attr_opt(node, "minOccurs")?,
            max_uccors: value_from_attr_opt(node, "maxOccurs")?,
            contents: first_child_of_type(node)?,
        })
    }
}

#[derive(Debug)]
pub enum XsdFileItem {
    SimpleType(SimpleType),
    ComplexType(ComplexType),
    Element(Element),
}

impl<'input> XmlLoad<'input> for Option<XsdFileItem> {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Some(match node.tag_name().name() {
            "simpleType" => XsdFileItem::SimpleType(XmlLoad::load(node)?),
            "complexType" => XsdFileItem::ComplexType(XmlLoad::load(node)?),
            "element" => XsdFileItem::Element(XmlLoad::load(node)?),
            _ => return Ok(None),
        }))
    }
}

#[derive(Debug)]
pub struct XmlSchema {
    pub items: Vec<XsdFileItem>,
    pub target_namespace: Option<String>,
    pub version: Option<String>,
}

impl<'input> XmlLoad<'input> for XmlSchema {
    fn load(node: &Node<'_, 'input>) -> Result<Self, XmlError> {
        Ok(Self {
            items: children_of_type(node)?,
            target_namespace: value_from_attr_opt(node, "targetNamespace")?,
            version: value_from_attr_opt(node, "version")?,
        })
    }
}

#[derive(Debug)]
pub enum XsdFileType {
    Simple(SimpleType),
    Complex(ComplexType),
}
