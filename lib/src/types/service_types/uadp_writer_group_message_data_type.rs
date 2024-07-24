// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct UadpWriterGroupMessageDataType {
    pub group_version: u32,
    pub data_set_ordering: super::enums::DataSetOrderingType,
    pub network_message_content_mask: super::enums::UadpNetworkMessageContentMask,
    pub sampling_offset: f64,
    pub publishing_offset: Option<Vec<f64>>,
}
impl crate::types::BinaryEncoder<UadpWriterGroupMessageDataType>
for UadpWriterGroupMessageDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.group_version.byte_len();
        size += self.data_set_ordering.byte_len();
        size += self.network_message_content_mask.byte_len();
        size += self.sampling_offset.byte_len();
        size += self.publishing_offset.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.group_version.encode(stream)?;
        size += self.data_set_ordering.encode(stream)?;
        size += self.network_message_content_mask.encode(stream)?;
        size += self.sampling_offset.encode(stream)?;
        size += self.publishing_offset.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let group_version = <u32 as crate::types::BinaryEncoder<
            u32,
        >>::decode(stream, decoding_options)?;
        let data_set_ordering = <super::enums::DataSetOrderingType as crate::types::BinaryEncoder<
            super::enums::DataSetOrderingType,
        >>::decode(stream, decoding_options)?;
        let network_message_content_mask = <super::enums::UadpNetworkMessageContentMask as crate::types::BinaryEncoder<
            super::enums::UadpNetworkMessageContentMask,
        >>::decode(stream, decoding_options)?;
        let sampling_offset = <f64 as crate::types::BinaryEncoder<
            f64,
        >>::decode(stream, decoding_options)?;
        let publishing_offset = <Option<
            Vec<f64>,
        > as crate::types::BinaryEncoder<
            Option<Vec<f64>>,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            group_version,
            data_set_ordering,
            network_message_content_mask,
            sampling_offset,
            publishing_offset,
        })
    }
}
