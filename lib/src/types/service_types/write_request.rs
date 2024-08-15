// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WriteRequest {
    pub request_header: crate::types::request_header::RequestHeader,
    pub nodes_to_write: Option<Vec<super::write_value::WriteValue>>,
}
impl crate::types::MessageInfo for WriteRequest {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::WriteRequest_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<WriteRequest> for WriteRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.nodes_to_write.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.nodes_to_write.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let request_header =
            <crate::types::request_header::RequestHeader as crate::types::BinaryEncoder<
                crate::types::request_header::RequestHeader,
            >>::decode(stream, decoding_options)?;
        let nodes_to_write =
            <Option<Vec<super::write_value::WriteValue>> as crate::types::BinaryEncoder<
                Option<Vec<super::write_value::WriteValue>>,
            >>::decode(stream, decoding_options)?;
        Ok(Self {
            request_header,
            nodes_to_write,
        })
    }
}
