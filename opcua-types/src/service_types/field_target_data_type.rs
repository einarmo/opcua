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
pub struct FieldTargetDataType {
    pub data_set_field_id: opcua::types::guid::Guid,
    pub receiver_index_range: opcua::types::string::UAString,
    pub target_node_id: opcua::types::node_id::NodeId,
    pub attribute_id: u32,
    pub write_index_range: opcua::types::string::UAString,
    pub override_value_handling: super::enums::OverrideValueHandling,
    pub override_value: opcua::types::variant::Variant,
}
impl opcua::types::MessageInfo for FieldTargetDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FieldTargetDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FieldTargetDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FieldTargetDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for FieldTargetDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.data_set_field_id.byte_len();
        size += self.receiver_index_range.byte_len();
        size += self.target_node_id.byte_len();
        size += self.attribute_id.byte_len();
        size += self.write_index_range.byte_len();
        size += self.override_value_handling.byte_len();
        size += self.override_value.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.data_set_field_id.encode(stream)?;
        size += self.receiver_index_range.encode(stream)?;
        size += self.target_node_id.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.write_index_range.encode(stream)?;
        size += self.override_value_handling.encode(stream)?;
        size += self.override_value.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for FieldTargetDataType {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            data_set_field_id: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            receiver_index_range: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            target_node_id: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            attribute_id: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            write_index_range: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            override_value_handling: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            override_value: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
