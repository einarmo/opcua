use std::{
    num::{ParseFloatError, ParseIntError},
    ops::Range,
    str::ParseBoolError,
};

use chrono::ParseError;
use roxmltree::Node;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum XmlErrorInner {
    #[error("Failed to load XML: {0}")]
    XML(#[from] roxmltree::Error),
    #[error("Expected child: {0}")]
    MissingField(String),
    #[error("Expected attribute: {0}")]
    MissingAttribute(String),
    #[error("Failed to parse {0} as integer.")]
    ParseInt(String, ParseIntError),
    #[error("Failed to parse {0} as float.")]
    ParseFloat(String, ParseFloatError),
    #[error("Failed to parse {0} as bool.")]
    ParseBool(String, ParseBoolError),
    #[error("Missing node content")]
    MissingContent,
    #[error("Invalid timestamp for {0}: {1}")]
    ParseDateTime(String, ParseError),
    #[error("Invalid UUID for {0}: {1}")]
    ParseUuid(String, uuid::Error),
    #[error("{0}")]
    Other(String),
}

#[derive(Error, Debug)]
#[error("{error} at {span:?}")]
pub struct XmlError {
    pub span: Range<usize>,
    pub error: XmlErrorInner,
}

impl XmlError {
    pub fn missing_field(node: &Node<'_, '_>, name: &str) -> Self {
        Self {
            span: node.range(),
            error: XmlErrorInner::MissingField(name.to_owned()),
        }
    }

    pub fn missing_attribute(node: &Node<'_, '_>, name: &str) -> Self {
        Self {
            span: node.range(),
            error: XmlErrorInner::MissingAttribute(name.to_owned()),
        }
    }

    pub fn other(node: &Node<'_, '_>, info: &str) -> Self {
        Self {
            span: node.range(),
            error: XmlErrorInner::Other(info.to_owned()),
        }
    }

    pub fn parse_int(node: &Node<'_, '_>, attr: &str, err: ParseIntError) -> Self {
        Self {
            span: node.range(),
            error: XmlErrorInner::ParseInt(attr.to_owned(), err),
        }
    }

    pub fn parse_float(node: &Node<'_, '_>, attr: &str, err: ParseFloatError) -> Self {
        Self {
            span: node.range(),
            error: XmlErrorInner::ParseFloat(attr.to_owned(), err),
        }
    }

    pub fn parse_bool(node: &Node<'_, '_>, attr: &str, err: ParseBoolError) -> Self {
        Self {
            span: node.range(),
            error: XmlErrorInner::ParseBool(attr.to_owned(), err),
        }
    }

    pub fn parse_date_time(node: &Node<'_, '_>, attr: &str, err: ParseError) -> Self {
        Self {
            span: node.range(),
            error: XmlErrorInner::ParseDateTime(attr.to_owned(), err),
        }
    }

    pub fn parse_uuid(node: &Node<'_, '_>, attr: &str, err: uuid::Error) -> Self {
        Self {
            span: node.range(),
            error: XmlErrorInner::ParseUuid(attr.to_owned(), err),
        }
    }

    pub fn missing_content(node: &Node<'_, '_>) -> Self {
        println!("{:?}", node.tag_name().name());
        Self {
            span: node.range(),
            error: XmlErrorInner::MissingContent,
        }
    }
}
