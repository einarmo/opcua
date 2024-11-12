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
pub struct RegisterServer2Response {
    pub response_header: opcua::types::response_header::ResponseHeader,
    pub configuration_results: Option<Vec<opcua::types::status_code::StatusCode>>,
    pub diagnostic_infos: Option<Vec<opcua::types::diagnostic_info::DiagnosticInfo>>,
}
impl opcua::types::MessageInfo for RegisterServer2Response {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::RegisterServer2Response_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::RegisterServer2Response_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::RegisterServer2Response_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for RegisterServer2Response {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size += self.configuration_results.byte_len();
        size += self.diagnostic_infos.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        size += self.configuration_results.encode(stream)?;
        size += self.diagnostic_infos.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for RegisterServer2Response {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let response_header: opcua::types::response_header::ResponseHeader = opcua::types::BinaryDecodable::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = response_header.request_handle;
        Ok(Self {
            response_header,
            configuration_results: opcua::types::BinaryDecodable::decode(
                    stream,
                    decoding_options,
                )
                .map_err(|e| e.with_request_handle(__request_handle))?,
            diagnostic_infos: opcua::types::BinaryDecodable::decode(
                    stream,
                    decoding_options,
                )
                .map_err(|e| e.with_request_handle(__request_handle))?,
        })
    }
}
