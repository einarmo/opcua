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
pub struct TranslateBrowsePathsToNodeIdsRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub browse_paths: Option<Vec<super::browse_path::BrowsePath>>,
}
impl opcua::types::MessageInfo for TranslateBrowsePathsToNodeIdsRequest {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::TranslateBrowsePathsToNodeIdsRequest_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for TranslateBrowsePathsToNodeIdsRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.browse_paths.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.browse_paths.encode(stream)?;
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
        let browse_paths = <Option<
            Vec<super::browse_path::BrowsePath>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            request_header,
            browse_paths,
        })
    }
}
