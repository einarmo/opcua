// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryReadRequest {
    pub request_header: crate::types::request_header::RequestHeader,
    pub history_read_details: crate::types::extension_object::ExtensionObject,
    pub timestamps_to_return: super::enums::TimestampsToReturn,
    pub release_continuation_points: bool,
    pub nodes_to_read: Option<Vec<super::history_read_value_id::HistoryReadValueId>>,
}
impl crate::types::MessageInfo for HistoryReadRequest {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::HistoryReadRequest_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<HistoryReadRequest> for HistoryReadRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.history_read_details.byte_len();
        size += self.timestamps_to_return.byte_len();
        size += self.release_continuation_points.byte_len();
        size += self.nodes_to_read.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.history_read_details.encode(stream)?;
        size += self.timestamps_to_return.encode(stream)?;
        size += self.release_continuation_points.encode(stream)?;
        size += self.nodes_to_read.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let request_header = <crate::types::request_header::RequestHeader as crate::types::BinaryEncoder<
            crate::types::request_header::RequestHeader,
        >>::decode(stream, decoding_options)?;
        let history_read_details = <crate::types::extension_object::ExtensionObject as crate::types::BinaryEncoder<
            crate::types::extension_object::ExtensionObject,
        >>::decode(stream, decoding_options)?;
        let timestamps_to_return = <super::enums::TimestampsToReturn as crate::types::BinaryEncoder<
            super::enums::TimestampsToReturn,
        >>::decode(stream, decoding_options)?;
        let release_continuation_points = <bool as crate::types::BinaryEncoder<
            bool,
        >>::decode(stream, decoding_options)?;
        let nodes_to_read = <Option<
            Vec<super::history_read_value_id::HistoryReadValueId>,
        > as crate::types::BinaryEncoder<
            Option<Vec<super::history_read_value_id::HistoryReadValueId>>,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            request_header,
            history_read_details,
            timestamps_to_return,
            release_continuation_points,
            nodes_to_read,
        })
    }
}
