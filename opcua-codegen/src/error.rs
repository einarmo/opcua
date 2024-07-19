use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeGenError<'a> {
    #[error("Failed to load XML: {0}")]
    XML(#[from] roxmltree::Error),
    #[error("Expected child: {0}")]
    MissingField(&'a str),
    #[error("Expected attribute: {0}")]
    MissingAttribute(&'a str),
    #[error("Wrong format on field. Expected {0}, got {1}")]
    WrongFormat(String, String),
    #[error("Failed to parse {0} as integer.")]
    ParseInt(String, ParseIntError),
    #[error("{0}")]
    Other(String),
    #[error("Failed to generate code: {0}")]
    Syn(#[from] syn::Error),
}
