// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct DeleteEventDetails {
    pub node_id: opcua::types::node_id::NodeId,
    pub event_ids: Option<Vec<opcua::types::byte_string::ByteString>>,
}
impl opcua::types::MessageInfo for DeleteEventDetails {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DeleteEventDetails_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DeleteEventDetails_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DeleteEventDetails_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for DeleteEventDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.node_id.byte_len();
        size += self.event_ids.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.node_id.encode(stream)?;
        size += self.event_ids.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for DeleteEventDetails {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            node_id: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            event_ids: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
        })
    }
}
