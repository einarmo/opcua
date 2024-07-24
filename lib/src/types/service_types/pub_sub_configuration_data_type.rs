// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
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
impl crate::types::MessageInfo for PubSubConfigurationDataType {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::PubSubConfigurationDataType_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<PubSubConfigurationDataType>
for PubSubConfigurationDataType {
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
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.published_data_sets.encode(stream)?;
        size += self.connections.encode(stream)?;
        size += self.enabled.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let published_data_sets = <Option<
            Vec<super::published_data_set_data_type::PublishedDataSetDataType>,
        > as crate::types::BinaryEncoder<
            Option<Vec<super::published_data_set_data_type::PublishedDataSetDataType>>,
        >>::decode(stream, decoding_options)?;
        let connections = <Option<
            Vec<super::pub_sub_connection_data_type::PubSubConnectionDataType>,
        > as crate::types::BinaryEncoder<
            Option<Vec<super::pub_sub_connection_data_type::PubSubConnectionDataType>>,
        >>::decode(stream, decoding_options)?;
        let enabled = <bool as crate::types::BinaryEncoder<
            bool,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            published_data_sets,
            connections,
            enabled,
        })
    }
}
