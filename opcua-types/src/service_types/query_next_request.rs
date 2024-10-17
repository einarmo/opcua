// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct QueryNextRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub release_continuation_point: bool,
    pub continuation_point: opcua::types::byte_string::ByteString,
}
impl opcua::types::MessageInfo for QueryNextRequest {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::QueryNextRequest_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for QueryNextRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.release_continuation_point.byte_len();
        size += self.continuation_point.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.release_continuation_point.encode(stream)?;
        size += self.continuation_point.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let request_header = <opcua::types::request_header::RequestHeader as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = request_header.request_handle;
        let release_continuation_point = <bool as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let continuation_point = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            request_header,
            release_continuation_point,
            continuation_point,
        })
    }
}
