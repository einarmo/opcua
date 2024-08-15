use std::{num::ParseIntError, ops::Range};

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
}
