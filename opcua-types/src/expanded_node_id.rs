// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock

//! Contains the implementation of `ExpandedNodeId`.

use std::{
    self,
    borrow::Cow,
    fmt,
    io::{Read, Write},
    str::FromStr,
};

use log::error;

use crate::{
    byte_string::ByteString,
    encoding::*,
    guid::Guid,
    node_id::{Identifier, NodeId},
    status_code::StatusCode,
    string::*,
    NamespaceMap,
};

/// A NodeId that allows the namespace URI to be specified instead of an index.
#[derive(PartialEq, Debug, Clone, Eq, Hash, Default)]
pub struct ExpandedNodeId {
    pub node_id: NodeId,
    pub namespace_uri: UAString,
    pub server_index: u32,
}

#[cfg(feature = "json")]
mod json {
    use serde::{
        de::{self, IgnoredAny, Visitor},
        ser::SerializeStruct,
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use serde_json::Value;

    use crate::{Identifier, NodeId};

    use super::ExpandedNodeId;
    // JSON serialization schema as per spec:
    //
    // "Type"
    //      The IdentifierType encoded as a JSON number.
    //      Allowed values are:
    //            0 - UInt32 Identifier encoded as a JSON number.
    //            1 - A String Identifier encoded as a JSON string.
    //            2 - A Guid Identifier encoded as described in 5.4.2.7.
    //            3 - A ByteString Identifier encoded as described in 5.4.2.8.
    //      This field is omitted for UInt32 identifiers.
    // "Id"
    //      The Identifier.
    //      The value of the id field specifies the encoding of this field.
    // "Namespace"
    //      The NamespaceIndex for the NodeId.
    //      The field is encoded as a JSON number for the reversible encoding.
    //      The field is omitted if the NamespaceIndex equals 0.
    //      For the non-reversible encoding, the field is the NamespaceUri associated with the NamespaceIndex, encoded as a JSON string.
    //      A NamespaceIndex of 1 is always encoded as a JSON number.
    // "ServerUri"
    //      The ServerIndex for the ExpandedNodeId.
    //      This field is encoded as a JSON number for the reversible encoding.
    //      This field is omitted if the ServerIndex equals 0.
    //      For the non-reversible encoding, this field is the ServerUri associated with the ServerIndex portion of the ExpandedNodeId, encoded as a JSON string.

    impl Serialize for ExpandedNodeId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut len = 1;
            if self.node_id.namespace != 0 || !self.namespace_uri.is_null() {
                len += 1;
            }
            if !matches!(self.node_id.identifier, Identifier::Numeric(_)) {
                len += 1;
            }
            if self.server_index != 0 {
                len += 1;
            }

            let mut struct_ser = serializer.serialize_struct("NodeId", len)?;
            match &self.node_id.identifier {
                Identifier::Numeric(n) => {
                    struct_ser.serialize_field("Id", n)?;
                }
                Identifier::String(uastring) => {
                    struct_ser.serialize_field("IdType", &1)?;
                    struct_ser.serialize_field("Id", uastring)?;
                }
                Identifier::Guid(guid) => {
                    struct_ser.serialize_field("IdType", &2)?;
                    struct_ser.serialize_field("Id", guid)?;
                }
                Identifier::ByteString(byte_string) => {
                    struct_ser.serialize_field("IdType", &3)?;
                    struct_ser.serialize_field("Id", byte_string)?;
                }
            }

            if !self.namespace_uri.is_null() {
                struct_ser.serialize_field("Namespace", self.namespace_uri.as_ref())?;
            } else if self.node_id.namespace != 0 {
                struct_ser.serialize_field("Namespace", &self.node_id.namespace)?;
            }
            if self.server_index != 0 {
                struct_ser.serialize_field("ServerUri", &self.server_index)?;
            }

            struct_ser.end()
        }
    }

    impl<'de> Deserialize<'de> for ExpandedNodeId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct ExpandedNodeIdVisitor;

            impl<'de> Visitor<'de> for ExpandedNodeIdVisitor {
                type Value = ExpandedNodeId;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, "an object containing a NodeId")
                }

                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
                {
                    let mut id_type: Option<u16> = None;
                    let mut namespace: Option<serde_json::Value> = None;
                    let mut value: Option<serde_json::Value> = None;
                    let mut server_uri: Option<u32> = None;
                    while let Some(key) = map.next_key::<String>()? {
                        match key.as_str() {
                            "Id" => {
                                value = Some(map.next_value()?);
                            }
                            "Namespace" => {
                                namespace = Some(map.next_value()?);
                            }
                            "IdType" => id_type = Some(map.next_value()?),
                            "ServerUri" => server_uri = Some(map.next_value()?),
                            _ => {
                                map.next_value::<IgnoredAny>()?;
                            }
                        }
                    }

                    // The standad implies that this field is required.
                    let Some(value) = value else {
                        return Err(de::Error::custom(
                            "Failed to deserialize NodeId, missing Id field",
                        ));
                    };

                    let identifier = match id_type {
                        Some(1) => Identifier::String(
                            serde_json::from_value(value).map_err(de::Error::custom)?,
                        ),
                        Some(2) => Identifier::Guid(
                            serde_json::from_value(value).map_err(de::Error::custom)?,
                        ),
                        Some(3) => Identifier::ByteString(
                            serde_json::from_value(value).map_err(de::Error::custom)?,
                        ),
                        None | Some(0) => Identifier::Numeric(
                            serde_json::from_value(value).map_err(de::Error::custom)?,
                        ),
                        Some(r) => {
                            return Err(de::Error::custom(format!(
                                "Failed to deserialize NodeId, got unexpected IdType {r}"
                            )))
                        }
                    };

                    let (namespace_uri, namespace) = match namespace {
                        Some(Value::String(s)) => (Some(s), 0),
                        Some(Value::Number(s)) => (None, s.as_u64().and_then(|v| v.try_into().ok())
                        .ok_or_else(|| de::Error::custom("Failed to deserialize ExpandedNodeId, expected 16-bit integer or string for Namespace"))?),
                        None => (None, 0),
                        _ => return Err(de::Error::custom("Failed to deserialize ExpandedNodeId, expected number or string for Namespace")),
                    };

                    Ok(ExpandedNodeId {
                        node_id: NodeId {
                            namespace,
                            identifier,
                        },
                        namespace_uri: namespace_uri.into(),
                        server_index: server_uri.unwrap_or_default(),
                    })
                }
            }

            deserializer.deserialize_map(ExpandedNodeIdVisitor)
        }
    }
}

impl BinaryEncodable for ExpandedNodeId {
    fn byte_len(&self) -> usize {
        let mut size = self.node_id.byte_len();
        if !self.namespace_uri.is_null() {
            size += self.namespace_uri.byte_len();
        }
        if self.server_index != 0 {
            size += self.server_index.byte_len();
        }
        size
    }

    fn encode<S: Write + ?Sized>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size: usize = 0;

        let mut data_encoding = 0;
        if !self.namespace_uri.is_null() {
            data_encoding |= 0x80;
        }
        if self.server_index != 0 {
            data_encoding |= 0x40;
        }

        // Type determines the byte code
        match &self.node_id.identifier {
            Identifier::Numeric(value) => {
                if self.node_id.namespace == 0 && *value <= 255 {
                    // node id fits into 2 bytes when the namespace is 0 and the value <= 255
                    size += write_u8(stream, data_encoding)?;
                    size += write_u8(stream, *value as u8)?;
                } else if self.node_id.namespace <= 255 && *value <= 65535 {
                    // node id fits into 4 bytes when namespace <= 255 and value <= 65535
                    size += write_u8(stream, data_encoding | 0x1)?;
                    size += write_u8(stream, self.node_id.namespace as u8)?;
                    size += write_u16(stream, *value as u16)?;
                } else {
                    // full node id
                    size += write_u8(stream, data_encoding | 0x2)?;
                    size += write_u16(stream, self.node_id.namespace)?;
                    size += write_u32(stream, *value)?;
                }
            }
            Identifier::String(value) => {
                size += write_u8(stream, data_encoding | 0x3)?;
                size += write_u16(stream, self.node_id.namespace)?;
                size += value.encode(stream)?;
            }
            Identifier::Guid(value) => {
                size += write_u8(stream, data_encoding | 0x4)?;
                size += write_u16(stream, self.node_id.namespace)?;
                size += value.encode(stream)?;
            }
            Identifier::ByteString(ref value) => {
                size += write_u8(stream, data_encoding | 0x5)?;
                size += write_u16(stream, self.node_id.namespace)?;
                size += value.encode(stream)?;
            }
        }
        if !self.namespace_uri.is_null() {
            size += self.namespace_uri.encode(stream)?;
        }
        if self.server_index != 0 {
            size += self.server_index.encode(stream)?;
        }
        assert_eq!(size, self.byte_len());
        Ok(size)
    }
}

impl BinaryDecodable for ExpandedNodeId {
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let data_encoding = read_u8(stream)?;
        let identifier = data_encoding & 0x0f;
        let node_id = match identifier {
            0x0 => {
                let value = read_u8(stream)?;
                NodeId::new(0, u32::from(value))
            }
            0x1 => {
                let namespace = read_u8(stream)?;
                let value = read_u16(stream)?;
                NodeId::new(u16::from(namespace), u32::from(value))
            }
            0x2 => {
                let namespace = read_u16(stream)?;
                let value = read_u32(stream)?;
                NodeId::new(namespace, value)
            }
            0x3 => {
                let namespace = read_u16(stream)?;
                let value = UAString::decode(stream, decoding_options)?;
                NodeId::new(namespace, value)
            }
            0x4 => {
                let namespace = read_u16(stream)?;
                let value = Guid::decode(stream, decoding_options)?;
                NodeId::new(namespace, value)
            }
            0x5 => {
                let namespace = read_u16(stream)?;
                let value = ByteString::decode(stream, decoding_options)?;
                NodeId::new(namespace, value)
            }
            _ => {
                error!("Unrecognized expanded node id type {}", identifier);
                return Err(StatusCode::BadDecodingError.into());
            }
        };

        // Optional stuff
        let namespace_uri = if data_encoding & 0x80 != 0 {
            UAString::decode(stream, decoding_options)?
        } else {
            UAString::null()
        };
        let server_index = if data_encoding & 0x40 != 0 {
            u32::decode(stream, decoding_options)?
        } else {
            0
        };

        Ok(ExpandedNodeId {
            node_id,
            namespace_uri,
            server_index,
        })
    }
}

impl From<&NodeId> for ExpandedNodeId {
    fn from(value: &NodeId) -> Self {
        value.clone().into()
    }
}

impl From<(NodeId, u32)> for ExpandedNodeId {
    fn from(v: (NodeId, u32)) -> Self {
        ExpandedNodeId {
            node_id: v.0,
            namespace_uri: UAString::null(),
            server_index: v.1,
        }
    }
}

impl From<(NodeId, &str)> for ExpandedNodeId {
    fn from(v: (NodeId, &str)) -> Self {
        ExpandedNodeId {
            node_id: v.0,
            namespace_uri: v.1.into(),
            server_index: 0,
        }
    }
}

impl From<NodeId> for ExpandedNodeId {
    fn from(v: NodeId) -> Self {
        ExpandedNodeId {
            node_id: v,
            namespace_uri: UAString::null(),
            server_index: 0,
        }
    }
}

impl fmt::Display for ExpandedNodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Formatted depending on the namespace uri being empty or not.
        if self.namespace_uri.is_empty() {
            // svr=<serverindex>;ns=<namespaceindex>;<type>=<value>
            write!(f, "svr={};{}", self.server_index, self.node_id)
        } else {
            // The % and ; chars have to be escaped out in the uri
            let namespace_uri = String::from(self.namespace_uri.as_ref())
                .replace('%', "%25")
                .replace(';', "%3b");
            // svr=<serverindex>;nsu=<uri>;<type>=<value>
            write!(
                f,
                "svr={};nsu={};{}",
                self.server_index, namespace_uri, self.node_id.identifier
            )
        }
    }
}

impl FromStr for ExpandedNodeId {
    type Err = StatusCode;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use regex::Regex;

        // Parses a node from a string using the format specified in 5.3.1.11 part 6
        //
        // svr=<serverindex>;ns=<namespaceindex>;<type>=<value>
        // or
        // svr=<serverindex>;nsu=<uri>;<type>=<value>

        lazy_static::lazy_static! {
            // Contains capture groups "svr", either "ns" or "nsu" and then "t" for type
            static ref RE: Regex = Regex::new(r"^svr=(?P<svr>[0-9]+);(ns=(?P<ns>[0-9]+)|nsu=(?P<nsu>[^;]+));(?P<t>[isgb]=.+)$").unwrap();
        }

        let captures = RE.captures(s).ok_or(StatusCode::BadNodeIdInvalid)?;

        // Server index
        let server_index = captures
            .name("svr")
            .ok_or(StatusCode::BadNodeIdInvalid)
            .and_then(|server_index| {
                server_index
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| StatusCode::BadNodeIdInvalid)
            })?;

        // Check for namespace uri
        let namespace_uri = if let Some(nsu) = captures.name("nsu") {
            // The % and ; chars need to be unescaped
            let nsu = String::from(nsu.as_str())
                .replace("%3b", ";")
                .replace("%25", "%");
            UAString::from(nsu)
        } else {
            UAString::null()
        };

        let namespace = if let Some(ns) = captures.name("ns") {
            ns.as_str()
                .parse::<u16>()
                .map_err(|_| StatusCode::BadNodeIdInvalid)?
        } else {
            0
        };

        // Type identifier
        let t = captures.name("t").unwrap();
        Identifier::from_str(t.as_str())
            .map(|t| ExpandedNodeId {
                server_index,
                namespace_uri,
                node_id: NodeId::new(namespace, t),
            })
            .map_err(|_| StatusCode::BadNodeIdInvalid)
    }
}

impl ExpandedNodeId {
    /// Creates an expanded node id from a node id
    pub fn new<T>(value: T) -> ExpandedNodeId
    where
        T: 'static + Into<ExpandedNodeId>,
    {
        value.into()
    }

    pub fn null() -> ExpandedNodeId {
        Self::new(NodeId::null())
    }

    pub fn is_null(&self) -> bool {
        self.node_id.is_null()
    }

    /// Try to resolve the expanded node ID into a NodeId.
    /// This will directly return the inner NodeId if namespace URI is null, otherwise it will
    /// try to return a NodeId with the namespace index given by the namespace uri.
    /// If server index is non-zero, this will always return None, otherwise, it will return
    /// None if the namespace is not in the namespace map.
    pub fn try_resolve<'a>(&'a self, namespaces: &NamespaceMap) -> Option<Cow<'a, NodeId>> {
        if self.server_index != 0 {
            return None;
        }
        if let Some(uri) = self.namespace_uri.value() {
            let idx = namespaces.get_index(uri)?;
            Some(Cow::Owned(NodeId {
                namespace: idx,
                identifier: self.node_id.identifier.clone(),
            }))
        } else {
            Some(Cow::Borrowed(&self.node_id))
        }
    }
}
