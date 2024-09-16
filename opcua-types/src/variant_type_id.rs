// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

use log::error;

use crate::{DataTypeId, NodeId, NodeIdError, StatusCode};

/// The variant type id is the type of the variant but without its payload.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VariantTypeId<'a> {
    Empty,
    Scalar(VariantScalarTypeId),
    Array(VariantScalarTypeId, Option<&'a [u32]>),
}

impl<'a> From<VariantScalarTypeId> for VariantTypeId<'a> {
    fn from(value: VariantScalarTypeId) -> Self {
        Self::Scalar(value)
    }
}

impl<'a> From<(VariantScalarTypeId, &'a [u32])> for VariantTypeId<'a> {
    fn from(value: (VariantScalarTypeId, &'a [u32])) -> Self {
        Self::Array(value.0, Some(value.1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VariantScalarTypeId {
    Boolean,
    SByte,
    Byte,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float,
    Double,
    String,
    DateTime,
    Guid,
    StatusCode,
    ByteString,
    XmlElement,
    QualifiedName,
    LocalizedText,
    NodeId,
    ExpandedNodeId,
    ExtensionObject,
    Variant,
    DataValue,
    DiagnosticInfo,
}

impl TryFrom<&NodeId> for VariantScalarTypeId {
    type Error = NodeIdError;
    fn try_from(value: &NodeId) -> Result<Self, NodeIdError> {
        let type_id = value.as_data_type_id()?;

        Ok(match type_id {
            DataTypeId::Boolean => Self::Boolean,
            DataTypeId::Byte => Self::Byte,
            DataTypeId::Int16 => Self::Int16,
            DataTypeId::UInt16 => Self::UInt16,
            DataTypeId::Int32 => Self::Int32,
            DataTypeId::UInt32 => Self::UInt32,
            DataTypeId::Int64 => Self::Int64,
            DataTypeId::UInt64 => Self::UInt64,
            DataTypeId::Float => Self::Float,
            DataTypeId::Double => Self::Double,
            DataTypeId::String => Self::String,
            DataTypeId::DateTime => Self::DateTime,
            DataTypeId::Guid => Self::Guid,
            DataTypeId::ByteString => Self::ByteString,
            DataTypeId::XmlElement => Self::XmlElement,
            DataTypeId::NodeId => Self::NodeId,
            DataTypeId::ExpandedNodeId => Self::ExpandedNodeId,
            DataTypeId::StatusCode => Self::StatusCode,
            DataTypeId::QualifiedName => Self::QualifiedName,
            DataTypeId::LocalizedText => Self::LocalizedText,
            DataTypeId::DataValue => Self::DataValue,
            DataTypeId::BaseDataType => Self::Variant,
            DataTypeId::DiagnosticInfo => Self::DiagnosticInfo,
            _ => return Err(NodeIdError),
        })
    }
}

impl<'a> TryFrom<&NodeId> for VariantTypeId<'a> {
    type Error = NodeIdError;
    fn try_from(value: &NodeId) -> Result<Self, NodeIdError> {
        Ok(Self::Scalar(VariantScalarTypeId::try_from(value)?))
    }
}

impl VariantScalarTypeId {
    pub fn encoding_mask(&self) -> u8 {
        match self {
            Self::Boolean => EncodingMask::BOOLEAN,
            Self::SByte => EncodingMask::SBYTE,
            Self::Byte => EncodingMask::BYTE,
            Self::Int16 => EncodingMask::INT16,
            Self::UInt16 => EncodingMask::UINT16,
            Self::Int32 => EncodingMask::INT32,
            Self::UInt32 => EncodingMask::UINT32,
            Self::Int64 => EncodingMask::INT64,
            Self::UInt64 => EncodingMask::UINT64,
            Self::Float => EncodingMask::FLOAT,
            Self::Double => EncodingMask::DOUBLE,
            Self::String => EncodingMask::STRING,
            Self::DateTime => EncodingMask::DATE_TIME,
            Self::Guid => EncodingMask::GUID,
            Self::StatusCode => EncodingMask::STATUS_CODE,
            Self::ByteString => EncodingMask::BYTE_STRING,
            Self::XmlElement => EncodingMask::XML_ELEMENT,
            Self::QualifiedName => EncodingMask::QUALIFIED_NAME,
            Self::LocalizedText => EncodingMask::LOCALIZED_TEXT,
            Self::NodeId => EncodingMask::NODE_ID,
            Self::ExpandedNodeId => EncodingMask::EXPANDED_NODE_ID,
            Self::ExtensionObject => EncodingMask::EXTENSION_OBJECT,
            Self::Variant => EncodingMask::VARIANT,
            Self::DataValue => EncodingMask::DATA_VALUE,
            Self::DiagnosticInfo => EncodingMask::DIAGNOSTIC_INFO,
        }
    }

    pub fn from_encoding_mask(encoding_mask: u8) -> Result<Self, StatusCode> {
        Ok(match encoding_mask & !EncodingMask::ARRAY_MASK {
            EncodingMask::BOOLEAN => Self::Boolean,
            EncodingMask::SBYTE => Self::SByte,
            EncodingMask::BYTE => Self::Byte,
            EncodingMask::INT16 => Self::Int16,
            EncodingMask::UINT16 => Self::UInt16,
            EncodingMask::INT32 => Self::Int32,
            EncodingMask::UINT32 => Self::UInt32,
            EncodingMask::INT64 => Self::Int64,
            EncodingMask::UINT64 => Self::UInt64,
            EncodingMask::FLOAT => Self::Float,
            EncodingMask::DOUBLE => Self::Double,
            EncodingMask::STRING => Self::String,
            EncodingMask::DATE_TIME => Self::DateTime,
            EncodingMask::GUID => Self::Guid,
            EncodingMask::STATUS_CODE => Self::StatusCode,
            EncodingMask::BYTE_STRING => Self::ByteString,
            EncodingMask::XML_ELEMENT => Self::XmlElement,
            EncodingMask::QUALIFIED_NAME => Self::QualifiedName,
            EncodingMask::LOCALIZED_TEXT => Self::LocalizedText,
            EncodingMask::NODE_ID => Self::NodeId,
            EncodingMask::EXPANDED_NODE_ID => Self::ExpandedNodeId,
            EncodingMask::EXTENSION_OBJECT => Self::ExtensionObject,
            EncodingMask::VARIANT => Self::Variant,
            EncodingMask::DATA_VALUE => Self::DataValue,
            EncodingMask::DIAGNOSTIC_INFO => Self::DiagnosticInfo,
            _ => {
                error!("Unrecognized encoding mask");
                return Err(StatusCode::BadDecodingError);
            }
        })
    }

    /// Tests and returns true if the variant holds a numeric type
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::SByte
                | Self::Byte
                | Self::Int16
                | Self::UInt16
                | Self::Int32
                | Self::UInt32
                | Self::Int64
                | Self::UInt64
                | Self::Float
                | Self::Double
        )
    }

    /// Returns a data precedence rank for scalar types, OPC UA part 4 table 119. This is used
    /// when operators are comparing values of differing types. The type with
    /// the highest precedence dictates how values are converted in order to be compared.
    pub fn precedence(&self) -> u8 {
        match self {
            Self::Double => 1,
            Self::Float => 2,
            Self::Int64 => 3,
            Self::UInt64 => 4,
            Self::Int32 => 5,
            Self::UInt32 => 6,
            Self::StatusCode => 7,
            Self::Int16 => 8,
            Self::UInt16 => 9,
            Self::SByte => 10,
            Self::Byte => 11,
            Self::Boolean => 12,
            Self::Guid => 13,
            Self::String => 14,
            Self::ExpandedNodeId => 15,
            Self::NodeId => 16,
            Self::LocalizedText => 17,
            Self::QualifiedName => 18,
            _ => 100,
        }
    }
}

impl<'a> VariantTypeId<'a> {
    pub fn encoding_mask(&self) -> u8 {
        match self {
            // Null / Empty
            VariantTypeId::Empty => 0u8,
            // Scalar types
            VariantTypeId::Scalar(s) => s.encoding_mask(),
            VariantTypeId::Array(s, dims) => {
                let mask = s.encoding_mask() | EncodingMask::ARRAY_VALUES_BIT;
                if dims.is_some() {
                    mask | EncodingMask::ARRAY_DIMENSIONS_BIT
                } else {
                    mask
                }
            }
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Self::Scalar(s) => s.precedence(),
            Self::Array(s, _) => s.precedence(),
            Self::Empty => 100,
        }
    }
}

pub(crate) struct EncodingMask;

impl EncodingMask {
    // These are values, not bits
    pub const BOOLEAN: u8 = DataTypeId::Boolean as u8;
    pub const SBYTE: u8 = DataTypeId::SByte as u8;
    pub const BYTE: u8 = DataTypeId::Byte as u8;
    pub const INT16: u8 = DataTypeId::Int16 as u8;
    pub const UINT16: u8 = DataTypeId::UInt16 as u8;
    pub const INT32: u8 = DataTypeId::Int32 as u8;
    pub const UINT32: u8 = DataTypeId::UInt32 as u8;
    pub const INT64: u8 = DataTypeId::Int64 as u8;
    pub const UINT64: u8 = DataTypeId::UInt64 as u8;
    pub const FLOAT: u8 = DataTypeId::Float as u8;
    pub const DOUBLE: u8 = DataTypeId::Double as u8;
    pub const STRING: u8 = DataTypeId::String as u8;
    pub const DATE_TIME: u8 = DataTypeId::DateTime as u8;
    pub const GUID: u8 = DataTypeId::Guid as u8;
    pub const BYTE_STRING: u8 = DataTypeId::ByteString as u8;
    pub const XML_ELEMENT: u8 = DataTypeId::XmlElement as u8;
    pub const NODE_ID: u8 = DataTypeId::NodeId as u8;
    pub const EXPANDED_NODE_ID: u8 = DataTypeId::ExpandedNodeId as u8;
    pub const STATUS_CODE: u8 = DataTypeId::StatusCode as u8;
    pub const QUALIFIED_NAME: u8 = DataTypeId::QualifiedName as u8;
    pub const LOCALIZED_TEXT: u8 = DataTypeId::LocalizedText as u8;
    pub const EXTENSION_OBJECT: u8 = 22; // DataTypeId::ExtensionObject as u8;
    pub const DATA_VALUE: u8 = DataTypeId::DataValue as u8;
    pub const VARIANT: u8 = 24;
    pub const DIAGNOSTIC_INFO: u8 = DataTypeId::DiagnosticInfo as u8;
    /// Bit indicates an array with dimensions
    pub const ARRAY_DIMENSIONS_BIT: u8 = 1 << 6;
    /// Bit indicates an array with values
    pub const ARRAY_VALUES_BIT: u8 = 1 << 7;

    pub const ARRAY_MASK: u8 = EncodingMask::ARRAY_DIMENSIONS_BIT | EncodingMask::ARRAY_VALUES_BIT;
}
