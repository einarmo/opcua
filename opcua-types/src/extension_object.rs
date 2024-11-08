// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! Contains the implementation of `ExtensionObject`.

use std::{
    error::Error,
    fmt,
    io::{Cursor, Read, Write},
};

use log::error;

use crate::{ExpandedMessageInfo, NamespaceMap};

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
    use serde::{
        de::{self, IgnoredAny, Visitor},
        ser::SerializeStruct,
        Deserialize, Serialize,
    };
    use serde_json::Value;

    use crate::{ByteString, ExtensionObjectEncoding, NodeId};

    use super::ExtensionObject;

    impl Serialize for ExtensionObject {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if matches!(self.body, ExtensionObjectEncoding::None) {
                return serializer.serialize_none();
            }

            let len = 3;

            let mut struct_ser = serializer.serialize_struct("ExtensionObject", len)?;
            struct_ser.serialize_field("TypeId", &self.node_id)?;
            match &self.body {
                ExtensionObjectEncoding::None => (),
                ExtensionObjectEncoding::ByteString(byte_string) => {
                    struct_ser.serialize_field("Encoding", &1)?;
                    struct_ser.serialize_field("Body", byte_string)?;
                }
                ExtensionObjectEncoding::XmlElement(uastring) => {
                    struct_ser.serialize_field("Encoding", &2)?;
                    struct_ser.serialize_field("Body", uastring)?;
                }
                ExtensionObjectEncoding::Json(json) => {
                    struct_ser.serialize_field("Body", json)?;
                }
            }

            struct_ser.end()
        }
    }

    impl<'de> Deserialize<'de> for ExtensionObject {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct ExtensionObjectVisitor;

            impl<'de> Visitor<'de> for ExtensionObjectVisitor {
                type Value = ExtensionObject;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, "an object containing an ExtensionObject")
                }

                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(ExtensionObject::null())
                }

                fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: de::Deserializer<'de>,
                {
                    deserializer.deserialize_map(self)
                }

                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
                {
                    let mut encoding: Option<u8> = None;
                    let mut type_id: Option<NodeId> = None;
                    let mut body: Option<Value> = None;

                    while let Some(key) = map.next_key::<String>()? {
                        match key.as_str() {
                            "TypeId" => {
                                type_id = Some(map.next_value()?);
                            }
                            "Encoding" => {
                                encoding = Some(map.next_value()?);
                            }
                            "Body" => {
                                body = Some(map.next_value()?);
                            }
                            _ => {
                                map.next_value::<IgnoredAny>()?;
                            }
                        }
                    }

                    // The standard is not super clear, for safety, it should be OK to just
                    // return a null object here, which is the most likely scenario.
                    let Some(type_id) = type_id else {
                        return Ok(ExtensionObject::null());
                    };

                    let encoding = encoding.unwrap_or_default();
                    let body = body.unwrap_or_default();

                    let body = match encoding {
                        0 => {
                            if body.is_null() {
                                ExtensionObjectEncoding::None
                            } else {
                                ExtensionObjectEncoding::Json(body)
                            }
                        }
                        1 => {
                            let Value::String(s) = body else {
                                return Err(de::Error::custom(
                                    "Expected a base64 serialized string as ExtensionObject body",
                                ));
                            };
                            ExtensionObjectEncoding::ByteString(ByteString::from_base64(&s).ok_or_else(|| de::Error::custom("Expected a base64 serialized string as ExtensionObject body"))?)
                        }
                        2 => {
                            let Value::String(s) = body else {
                                return Err(de::Error::custom(
                                    "Expected a JSON string as XML ExtensionObject body",
                                ));
                            };
                            ExtensionObjectEncoding::XmlElement(s.into())
                        }
                        r => {
                            return Err(de::Error::custom(format!(
                                "Expected 0, 1, or 2 as ExtensionObject encoding, got {r}"
                            )));
                        }
                    };

                    Ok(ExtensionObject {
                        node_id: type_id,
                        body,
                    })
                }
            }

            deserializer.deserialize_option(ExtensionObjectVisitor)
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

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
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
        T: BinaryEncodable,
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
