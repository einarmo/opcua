// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! Contains the implementation of `ExtensionObject`.

use std::{
    any::Any,
    error::Error,
    fmt,
    io::{Cursor, Read, Write},
};

use log::error;

use crate::{ExpandedMessageInfo, ExpandedNodeId, NamespaceMap};

use super::{
    byte_string::ByteString, encoding::*, node_id::NodeId, node_ids::ObjectId,
    status_code::StatusCode, string::XmlElement, MessageInfo,
};

#[derive(Debug)]
pub struct ExtensionObjectError;

impl fmt::Display for ExtensionObjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExtensionObjectError")
    }
}

pub trait DynEncodable: Any + std::fmt::Debug {
    fn encode_dyn(&self, stream: &mut dyn std::io::Write) -> EncodingResult<usize>;

    fn byte_len_dyn(&self) -> usize;

    fn binary_type_id(&self) -> ExpandedNodeId;
}

impl<T> DynEncodable for T
where
    T: BinaryEncodable + ExpandedMessageInfo + Any + std::fmt::Debug,
{
    fn encode_dyn(&self, stream: &mut dyn std::io::Write) -> EncodingResult<usize> {
        BinaryEncodable::encode(self, stream)
    }

    fn byte_len_dyn(&self) -> usize {
        BinaryEncodable::byte_len(self)
    }

    fn binary_type_id(&self) -> ExpandedNodeId {
        self.full_type_id()
    }
}

impl PartialEq for dyn DynEncodable {
    fn eq(&self, other: &dyn DynEncodable) -> bool {
        if self.byte_len_dyn() != other.byte_len_dyn() {
            return false;
        }
        if self.type_id() != other.type_id() {
            return false;
        }
        // For equality, just serialize both sides.
        let mut cursor = Vec::<u8>::with_capacity(self.byte_len_dyn());
        let mut cursor2 = Vec::<u8>::with_capacity(self.byte_len_dyn());

        if self.encode_dyn(&mut cursor).is_err() {
            return false;
        }

        if other.encode_dyn(&mut cursor2).is_err() {
            return false;
        }

        cursor == cursor2
    }
}

impl Error for ExtensionObjectError {}

/// Enumeration that holds the kinds of encoding that an ExtensionObject data may be encoded with.
#[derive(PartialEq, Debug, Clone)]
pub enum ExtensionObjectEncoding {
    /// For an extension object with nothing encoded with it
    None,
    /// For an extension object with data encoded in a ByteString
    ByteString(ByteString),
    /// For an extension object with data encoded in an XML string
    XmlElement(XmlElement),
    /// For an extension object with data encoded in a json string
    #[cfg(feature = "json")]
    Json(serde_json::Value),
}

#[cfg(feature = "json")]
mod json {
    use crate::json::*;

    use super::{ExtensionObject, ExtensionObjectEncoding};

    impl JsonEncodable for ExtensionObject {
        fn encode(
            &self,
            stream: &mut JsonStreamWriter<&mut dyn std::io::Write>,
            ctx: &crate::Context<'_>,
        ) -> super::EncodingResult<()> {
            if matches!(self.body, ExtensionObjectEncoding::None) {
                stream.null_value()?;
                return Ok(());
            }

            stream.begin_object()?;
            match &self.body {
                ExtensionObjectEncoding::None => unreachable!(),
                ExtensionObjectEncoding::ByteString(s) => {
                    stream.name("Encoding")?;
                    stream.number_value(1)?;
                    stream.name("Body")?;
                    JsonEncodable::encode(s, stream, ctx)?;
                }
                ExtensionObjectEncoding::XmlElement(s) => {
                    stream.name("Encoding")?;
                    stream.number_value(2)?;
                    stream.name("Body")?;
                    JsonEncodable::encode(s, stream, ctx)?;
                }
                ExtensionObjectEncoding::Json(s) => {
                    stream.name("Body")?;
                    // TODO
                    JsonEncodable::encode(&s.to_string(), stream, ctx)?;
                }
            }

            stream.end_object()?;

            Ok(())
        }
    }

    impl JsonDecodable for ExtensionObject {
        fn decode(
            stream: &mut JsonStreamReader<&mut dyn std::io::Read>,
            ctx: &Context<'_>,
        ) -> super::EncodingResult<Self> {
            if stream.peek()? == ValueType::Null {
                stream.next_null()?;
                return Ok(Self::null());
            }

            todo!();
        }
    }
}

/// An extension object holds a serialized object identified by its node id.
#[derive(PartialEq, Debug, Clone)]
pub struct ExtensionObject {
    pub node_id: NodeId,
    pub body: ExtensionObjectEncoding,
}

impl Default for ExtensionObject {
    fn default() -> Self {
        Self::null()
    }
}

impl BinaryEncodable for ExtensionObject {
    fn byte_len(&self) -> usize {
        let mut size = self.node_id.byte_len();
        size += match self.body {
            ExtensionObjectEncoding::None => 1,
            ExtensionObjectEncoding::ByteString(ref value) => {
                // Encoding mask + data
                1 + value.byte_len()
            }
            ExtensionObjectEncoding::XmlElement(ref value) => {
                // Encoding mask + data
                1 + value.byte_len()
            }
            #[cfg(feature = "json")]
            ExtensionObjectEncoding::Json(_) => {
                // Not really something we expect normally. Serialize it as encoding 0, i.e. nothing.
                1
            }
        };
        size
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        match self.body {
            ExtensionObjectEncoding::None => {
                size += write_u8(stream, 0x0)?;
            }
            ExtensionObjectEncoding::ByteString(ref value) => {
                // Encoding mask + data
                size += write_u8(stream, 0x1)?;
                size += value.encode(stream)?;
            }
            ExtensionObjectEncoding::XmlElement(ref value) => {
                // Encoding mask + data
                size += write_u8(stream, 0x2)?;
                size += value.encode(stream)?;
            }
            #[cfg(feature = "json")]
            ExtensionObjectEncoding::Json(_) => {
                // We don't support encoding a JSON extension object as binary. Serialize it as encoding 0, i.e. nothing
                size += write_u8(stream, 0x0)?;
            }
        }
        assert_eq!(size, self.byte_len());
        Ok(size)
    }
}
impl BinaryDecodable for ExtensionObject {
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        // Extension object is depth checked to prevent deep recursion
        let _depth_lock = decoding_options.depth_lock()?;
        let node_id = NodeId::decode(stream, decoding_options)?;
        let encoding_type = u8::decode(stream, decoding_options)?;
        let body = match encoding_type {
            0x0 => ExtensionObjectEncoding::None,
            0x1 => {
                ExtensionObjectEncoding::ByteString(ByteString::decode(stream, decoding_options)?)
            }
            0x2 => {
                ExtensionObjectEncoding::XmlElement(XmlElement::decode(stream, decoding_options)?)
            }
            _ => {
                error!("Invalid encoding type {} in stream", encoding_type);
                return Err(StatusCode::BadDecodingError.into());
            }
        };
        Ok(ExtensionObject { node_id, body })
    }
}

impl ExtensionObject {
    /// Creates a null extension object, i.e. one with no value or payload
    pub fn null() -> ExtensionObject {
        ExtensionObject {
            node_id: NodeId::null(),
            body: ExtensionObjectEncoding::None,
        }
    }

    /// Tests for null node id.
    pub fn is_null(&self) -> bool {
        self.node_id.is_null()
    }

    /// Tests for empty body.
    pub fn is_empty(&self) -> bool {
        self.is_null() || matches!(self.body, ExtensionObjectEncoding::None)
    }

    /// Returns the object id of the thing this extension object contains, assuming the
    /// object id can be recognised from the node id.
    pub fn object_id(&self) -> Result<ObjectId, ExtensionObjectError> {
        self.node_id
            .as_object_id()
            .map_err(|_| ExtensionObjectError)
    }

    /// Creates an extension object with the specified node id and the encodable object as its payload.
    /// The body is set to a byte string containing the encoded struct.
    pub fn from_encodable<N, T>(node_id: N, encodable: &T) -> ExtensionObject
    where
        N: Into<NodeId>,
        T: BinaryEncodable,
    {
        // Serialize to extension object
        let mut stream = Cursor::new(vec![0u8; encodable.byte_len()]);
        let _ = encodable.encode(&mut stream);
        ExtensionObject {
            node_id: node_id.into(),
            body: ExtensionObjectEncoding::ByteString(ByteString::from(stream.into_inner())),
        }
    }

    pub fn from_message<T>(encodable: &T) -> ExtensionObject
    where
        T: BinaryEncodable + MessageInfo,
    {
        Self::from_encodable(encodable.type_id(), encodable)
    }

    #[cfg(feature = "json")]
    pub fn from_json<T: serde::Serialize + MessageInfo>(
        object: &T,
    ) -> Result<ExtensionObject, serde_json::Error> {
        let value = serde_json::to_value(object)?;
        Ok(Self {
            node_id: object.json_type_id().into(),
            body: ExtensionObjectEncoding::Json(value),
        })
    }

    pub fn from_message_full<T>(
        encodable: &T,
        ctx: &NamespaceMap,
    ) -> Result<ExtensionObject, StatusCode>
    where
        T: BinaryEncodable + ExpandedMessageInfo,
    {
        let id = ctx
            .resolve_node_id(&encodable.full_type_id())
            .ok_or(StatusCode::BadNodeIdUnknown)?
            .into_owned();
        Ok(Self::from_encodable(id, encodable))
    }

    #[cfg(feature = "json")]
    pub fn from_json_full<T: serde::Serialize + ExpandedMessageInfo>(
        object: &T,
        ctx: &crate::EncodingContext,
    ) -> Result<ExtensionObject, serde_json::Error> {
        use serde::de::Error;

        let id = ctx
            .resolve_node_id(&object.full_type_id())
            .ok_or_else(|| serde_json::Error::custom("Encoding ID cannot be resolved"))?
            .into_owned();
        let value = serde_json::to_value(object)?;
        Ok(Self {
            node_id: id,
            body: ExtensionObjectEncoding::Json(value),
        })
    }

    /// Decodes the inner content of the extension object and returns it. The node id is ignored
    /// for decoding. The caller supplies the binary encoder impl that should be used to extract
    /// the data. Errors result in a decoding error.
    pub fn decode_inner<T>(&self, decoding_options: &DecodingOptions) -> EncodingResult<T>
    where
        T: BinaryDecodable,
    {
        match self.body {
            ExtensionObjectEncoding::ByteString(ref byte_string) => {
                if let Some(ref value) = byte_string.value {
                    // let value = value.clone();
                    let mut stream = Cursor::new(value);
                    T::decode(&mut stream, decoding_options)
                } else {
                    Err(StatusCode::BadDecodingError.into())
                }
            }
            _ => {
                error!("decode_inner called on an unsupported ExtensionObject type");
                Err(StatusCode::BadDecodingError.into())
            }
        }
    }
}
