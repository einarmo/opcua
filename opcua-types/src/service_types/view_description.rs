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
pub struct ViewDescription {
    pub view_id: opcua::types::node_id::NodeId,
    pub timestamp: opcua::types::date_time::DateTime,
    pub view_version: u32,
}
impl opcua::types::MessageInfo for ViewDescription {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ViewDescription_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ViewDescription_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ViewDescription_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for ViewDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.view_id.byte_len();
        size += self.timestamp.byte_len();
        size += self.view_version.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.view_id.encode(stream)?;
        size += self.timestamp.encode(stream)?;
        size += self.view_version.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for ViewDescription {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            view_id: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            timestamp: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            view_version: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
