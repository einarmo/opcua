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
pub struct ReadValueId {
    pub node_id: opcua::types::node_id::NodeId,
    pub attribute_id: u32,
    pub index_range: opcua::types::string::UAString,
    pub data_encoding: opcua::types::qualified_name::QualifiedName,
}
impl opcua::types::MessageInfo for ReadValueId {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadValueId_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadValueId_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadValueId_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for ReadValueId {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.node_id.byte_len();
        size += self.attribute_id.byte_len();
        size += self.index_range.byte_len();
        size += self.data_encoding.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.node_id.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        size += self.data_encoding.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for ReadValueId {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            node_id: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            attribute_id: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            index_range: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            data_encoding: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
