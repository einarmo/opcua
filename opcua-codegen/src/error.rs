use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("Failed to load XML: {0}")]
    XML(#[from] opcua_xml::XmlError),
    #[error("Missing required field: {0}")]
    MissingRequiredValue(&'static str),
    #[error("Wrong format on field. Expected {0}, got {1}")]
    WrongFormat(String, String),
    #[error("Failed to parse {0} as integer.")]
    ParseInt(String, ParseIntError),
    #[error("{0}")]
    Other(String),
    #[error("Failed to generate code: {0}")]
    Syn(#[from] syn::Error),
    #[error("Failed to load file: {0}")]
    Io(#[from] std::io::Error),
}
