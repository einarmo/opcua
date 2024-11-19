// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! Contains the implementation of `ExtensionObject`.

use std::{
    any::{Any, TypeId},
    error::Error,
    fmt,
    io::{Read, Write},
};

use log::error;

use crate::{ExpandedMessageInfo, ExpandedNodeId};

use super::{
    byte_string::ByteString, encoding::*, node_id::NodeId, node_ids::ObjectId,
    status_code::StatusCode, string::XmlElement,
};

#[derive(Debug)]
pub struct ExtensionObjectError;

impl fmt::Display for ExtensionObjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExtensionObjectError")
    }
}

pub trait DynEncodable: Any + Send + Sync + std::fmt::Debug {
    fn encode_binary(&self, stream: &mut dyn std::io::Write) -> EncodingResult<usize>;

    #[cfg(feature = "json")]
    fn encode_json(
        &self,
        stream: &mut crate::json::JsonStreamWriter<&mut dyn std::io::Write>,
        ctx: &crate::Context<'_>,
    ) -> EncodingResult<()>;

    fn byte_len_dyn(&self) -> usize;

    fn binary_type_id(&self) -> ExpandedNodeId;

    #[cfg(feature = "json")]
    fn json_type_id(&self) -> ExpandedNodeId;

    fn as_dyn_any(self: Box<Self>) -> Box<dyn Any + Send + Sync + 'static>;

    fn clone_box(&self) -> Box<dyn DynEncodable>;
}

macro_rules! blanket_dyn_encodable {
    ($bound:tt $(+ $others:tt)*) => {
        impl<T> DynEncodable for T
        where
            T: $bound  $(+ $others)* + ExpandedMessageInfo + Any + std::fmt::Debug + Send + Sync + Clone,
        {
            fn encode_binary(&self, stream: &mut dyn std::io::Write) -> EncodingResult<usize> {
                BinaryEncodable::encode(self, stream)
            }

            #[cfg(feature = "json")]
            fn encode_json(
                &self,
                stream: &mut crate::json::JsonStreamWriter<&mut dyn std::io::Write>,
                ctx: &crate::Context<'_>
            ) -> EncodingResult<()> {
                JsonEncodable::encode(self, stream, ctx)
            }

            fn byte_len_dyn(&self) -> usize {
                BinaryEncodable::byte_len(self)
            }

            fn binary_type_id(&self) -> ExpandedNodeId {
                self.full_type_id()
            }

            #[cfg(feature = "json")]
            fn json_type_id(&self) -> ExpandedNodeId {
                self.full_json_type_id()
            }

            fn as_dyn_any(self: Box<Self>) -> Box<dyn Any + Send + Sync + 'static> {
                self
            }

            fn clone_box(&self) -> Box<dyn DynEncodable> {
                Box::new(self.clone())
            }
        }
    };
}

#[cfg(feature = "json")]
use crate::json::JsonEncodable;

#[cfg(feature = "json")]
blanket_dyn_encodable!(BinaryEncodable + JsonEncodable);

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

        if self.encode_binary(&mut cursor).is_err() {
            return false;
        }

        if other.encode_binary(&mut cursor2).is_err() {
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
    use std::io::{Cursor, Read};

    use crate::{json::*, ByteString, NodeId, StatusCode};

    use super::ExtensionObject;

    impl JsonEncodable for ExtensionObject {
        fn encode(
            &self,
            stream: &mut JsonStreamWriter<&mut dyn std::io::Write>,
            ctx: &crate::Context<'_>,
        ) -> super::EncodingResult<()> {
            let Some(body) = &self.body else {
                stream.null_value()?;
                return Ok(());
            };

            let type_id = body.json_type_id();

            let id = type_id.try_resolve(ctx.namespaces()).ok_or_else(|| {
                log::warn!("Missing namespace for encoding ID: {}", type_id);
                StatusCode::BadDecodingError
            })?;

            stream.begin_object()?;

            stream.name("TypeId")?;
            JsonEncodable::encode(id.as_ref(), stream, ctx)?;

            stream.name("Body")?;
            body.encode_json(stream, ctx)?;

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

            let mut type_id: Option<NodeId> = None;
            let mut encoding: Option<u32> = None;
            let mut raw_body = None;
            let mut raw_binary_body: Option<ByteString> = None;
            let mut body = None;

            stream.begin_object()?;

            while stream.has_next()? {
                match stream.next_name()? {
                    "TypeId" => type_id = Some(JsonDecodable::decode(stream, ctx)?),
                    "Encoding" => encoding = Some(JsonDecodable::decode(stream, ctx)?),
                    "Body" => match stream.peek()? {
                        ValueType::Object => {
                            if encoding.is_some_and(|e| e != 0) {
                                log::warn!(
                                    "Invalid encoding, expected 0 or null, got {:?}",
                                    encoding
                                );
                                return Err(StatusCode::BadDecodingError.into());
                            }
                            if let Some(type_id) = &type_id {
                                body = Some(ctx.load_from_json(type_id, stream, ctx)?);
                            } else {
                                raw_body = Some(consume_raw_value(stream)?);
                            }
                        }
                        _ => {
                            if let Some(enc) = encoding {
                                if enc != 1 {
                                    log::warn!("Unsupported extension object encoding, expected 1 for string, got {enc}");
                                    return Err(StatusCode::BadDecodingError.into());
                                }
                            }
                            raw_binary_body = Some(JsonDecodable::decode(stream, ctx)?);
                        }
                    },
                    _ => stream.skip_value()?,
                }
            }

            stream.end_object()?;

            let Some(type_id) = type_id else {
                log::warn!("Missing type ID in extension object");
                return Err(StatusCode::BadDecodingError.into());
            };

            let encoding = encoding.unwrap_or_default();

            if let Some(body) = body {
                Ok(body)
            } else if let Some(raw_body) = raw_body {
                if encoding != 0 {
                    log::warn!("Invalid encoding, expected 0 or null, got {}", encoding);
                    return Err(StatusCode::BadDecodingError.into());
                }
                let mut cursor = Cursor::new(raw_body);
                let mut inner_stream = JsonStreamReader::new(&mut cursor as &mut dyn Read);
                Ok(ctx.load_from_json(&type_id, &mut inner_stream, ctx)?)
            } else if let Some(binary_body) = raw_binary_body {
                if encoding != 1 {
                    log::warn!(
                        "Unsupported extension object encoding, expected 1 for string, got {encoding}"
                    );
                    return Err(StatusCode::BadDecodingError.into());
                }
                let Some(raw) = binary_body.value else {
                    log::warn!("Missing extension object body");
                    return Err(StatusCode::BadDecodingError.into());
                };
                let mut cursor = Cursor::new(raw);
                Ok(ctx.load_from_binary(&type_id, &mut cursor as &mut dyn Read, ctx)?)
            } else {
                log::warn!("Missing extension object body");
                Err(StatusCode::BadDecodingError.into())
            }
        }
    }
}

/// An extension object holds a serialized object identified by its node id.
#[derive(PartialEq, Debug)]
pub struct ExtensionObject {
    pub body: Option<Box<dyn DynEncodable>>,
}

impl Clone for ExtensionObject {
    fn clone(&self) -> Self {
        Self {
            body: self.body.as_ref().map(|b| b.clone_box()),
        }
    }
}

impl Default for ExtensionObject {
    fn default() -> Self {
        Self::null()
    }
}

impl BinaryEncodable for ExtensionObject {
    fn byte_len(&self) -> usize {
        let mut size = self.node_id.byte_len();
        size += match &self.body {
            Some(b) => b.byte_len_dyn(),
            None => 1,
        };

        size
    }

    fn encode<S: Write + ?Sized>(&self, mut stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += BinaryEncodable::encode(&self.node_id, stream)?;

        match &self.body {
            Some(b) => {
                size += write_u8(stream, 0x1)?;
                size += b.encode_binary(&mut stream as &mut dyn Write)?;
            }
            None => {
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
        ExtensionObject { body: None }
    }

    /// Tests for null node id.
    pub fn is_null(&self) -> bool {
        self.body.is_none()
    }

    /// Tests for empty body.
    pub fn is_empty(&self) -> bool {
        self.is_null() || matches!(self.body, None)
    }

    pub fn binary_type_id(&self) -> ExpandedNodeId {
        self.body
            .as_ref()
            .map(|b| b.binary_type_id())
            .unwrap_or_else(|| ExpandedNodeId::null())
    }

    /// Returns the object id of the thing this extension object contains, assuming the
    /// object id can be recognised from the node id.
    pub fn object_id(&self) -> Result<ObjectId, ExtensionObjectError> {
        self.body
            .as_ref()
            .ok_or(ExtensionObjectError)?
            .binary_type_id()
            .node_id
            .as_object_id()
            .map_err(|_| ExtensionObjectError)
    }

    pub fn from_message<T>(encodable: T) -> ExtensionObject
    where
        T: DynEncodable,
    {
        Self {
            body: Some(Box::new(encodable)),
        }
    }

    pub fn inner_as<T: Send + Sync + 'static>(&self) -> Option<Box<T>> {
        if !self.inner_is::<T>() {
            return None;
        }

        self.body
            .as_ref()
            .and_then(|b| b.clone_box().as_dyn_any().downcast().ok())
    }

    pub fn type_id(&self) -> Option<TypeId> {
        self.body.as_ref().map(|b| b.type_id())
    }

    pub fn inner_is<T: 'static>(&self) -> bool {
        self.type_id() == Some(TypeId::of::<T>())
    }
}
