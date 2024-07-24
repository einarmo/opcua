// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct CloseSessionResponse {
    pub response_header: crate::types::response_header::ResponseHeader,
}
impl crate::types::MessageInfo for CloseSessionResponse {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::CloseSessionResponse_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<CloseSessionResponse> for CloseSessionResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let response_header = <crate::types::response_header::ResponseHeader as crate::types::BinaryEncoder<
            crate::types::response_header::ResponseHeader,
        >>::decode(stream, decoding_options)?;
        Ok(Self { response_header })
    }
}
