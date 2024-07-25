use chrono::{DateTime, Utc};

use crate::{ext::first_child_with_name_opt, XmlLoad};

pub enum OpcUaType {
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
    DateTime(DateTime<Utc>),
    ListOfDateTime(Vec<DateTime<Utc>>),
    Guid(String),
    ListOfGuid(Vec<String>),
    ByteString(String),
    ListOfByteString(Vec<String>),
    // Figure out XML elements somehow? Due to lifetimes we can't just pass around
    // roxmltree nodes.
    NodeId(NodeIdType),
    ListOfNodeId(Vec<NodeIdType>),
    ExpandedNodeId(NodeIdType),
    ListOfExpandedNodeId(Vec<NodeIdType>),
    StatusCode(StatusCode),
    ListOfStatusCode(Vec<StatusCode>),
}

#[derive(Debug)]
pub struct NodeIdType {
    pub identifier: Option<String>,
}

impl<'input> XmlLoad<'input> for NodeIdType {
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


