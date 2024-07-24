// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct CloseSessionRequest {
    pub request_header: crate::types::request_header::RequestHeader,
    pub delete_subscriptions: bool,
}
impl crate::types::MessageInfo for CloseSessionRequest {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::CloseSessionRequest_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<CloseSessionRequest> for CloseSessionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.delete_subscriptions.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.delete_subscriptions.encode(stream)?;
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
        let delete_subscriptions = <bool as crate::types::BinaryEncoder<
            bool,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            request_header,
            delete_subscriptions,
        })
    }
}
