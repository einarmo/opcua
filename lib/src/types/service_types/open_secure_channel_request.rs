// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct OpenSecureChannelRequest {
    pub request_header: crate::types::request_header::RequestHeader,
    pub client_protocol_version: u32,
    pub request_type: super::enums::SecurityTokenRequestType,
    pub security_mode: super::enums::MessageSecurityMode,
    pub client_nonce: crate::types::byte_string::ByteString,
    pub requested_lifetime: u32,
}
impl crate::types::MessageInfo for OpenSecureChannelRequest {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::OpenSecureChannelRequest_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<OpenSecureChannelRequest> for OpenSecureChannelRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.client_protocol_version.byte_len();
        size += self.request_type.byte_len();
        size += self.security_mode.byte_len();
        size += self.client_nonce.byte_len();
        size += self.requested_lifetime.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.client_protocol_version.encode(stream)?;
        size += self.request_type.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.client_nonce.encode(stream)?;
        size += self.requested_lifetime.encode(stream)?;
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
        let client_protocol_version = <u32 as crate::types::BinaryEncoder<
            u32,
        >>::decode(stream, decoding_options)?;
        let request_type = <super::enums::SecurityTokenRequestType as crate::types::BinaryEncoder<
            super::enums::SecurityTokenRequestType,
        >>::decode(stream, decoding_options)?;
        let security_mode = <super::enums::MessageSecurityMode as crate::types::BinaryEncoder<
            super::enums::MessageSecurityMode,
        >>::decode(stream, decoding_options)?;
        let client_nonce = <crate::types::byte_string::ByteString as crate::types::BinaryEncoder<
            crate::types::byte_string::ByteString,
        >>::decode(stream, decoding_options)?;
        let requested_lifetime = <u32 as crate::types::BinaryEncoder<
            u32,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            request_header,
            client_protocol_version,
            request_type,
            security_mode,
            client_nonce,
            requested_lifetime,
        })
    }
}
