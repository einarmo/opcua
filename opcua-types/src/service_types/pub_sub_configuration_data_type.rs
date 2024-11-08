// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct PubSubConfigurationDataType {
    pub published_data_sets: Option<
        Vec<super::published_data_set_data_type::PublishedDataSetDataType>,
    >,
    pub connections: Option<
        Vec<super::pub_sub_connection_data_type::PubSubConnectionDataType>,
    >,
    pub enabled: bool,
}
impl opcua::types::MessageInfo for PubSubConfigurationDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PubSubConfigurationDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PubSubConfigurationDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PubSubConfigurationDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for PubSubConfigurationDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.published_data_sets.byte_len();
        size += self.connections.byte_len();
        size += self.enabled.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.published_data_sets.encode(stream)?;
        size += self.connections.encode(stream)?;
        size += self.enabled.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            published_data_sets: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            connections: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            enabled: opcua::types::BinaryEncodable::decode(stream, decoding_options)?,
        })
    }
}
