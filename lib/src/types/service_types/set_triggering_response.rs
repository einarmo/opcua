// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct SetTriggeringResponse {
    pub response_header: crate::types::response_header::ResponseHeader,
    pub add_results: Option<Vec<crate::types::status_code::StatusCode>>,
    pub add_diagnostic_infos: Option<Vec<crate::types::diagnostic_info::DiagnosticInfo>>,
    pub remove_results: Option<Vec<crate::types::status_code::StatusCode>>,
    pub remove_diagnostic_infos: Option<
        Vec<crate::types::diagnostic_info::DiagnosticInfo>,
    >,
}
impl crate::types::MessageInfo for SetTriggeringResponse {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::SetTriggeringResponse_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<SetTriggeringResponse> for SetTriggeringResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size += self.add_results.byte_len();
        size += self.add_diagnostic_infos.byte_len();
        size += self.remove_results.byte_len();
        size += self.remove_diagnostic_infos.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        size += self.add_results.encode(stream)?;
        size += self.add_diagnostic_infos.encode(stream)?;
        size += self.remove_results.encode(stream)?;
        size += self.remove_diagnostic_infos.encode(stream)?;
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
        let add_results = <Option<
            Vec<crate::types::status_code::StatusCode>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::status_code::StatusCode>>,
        >>::decode(stream, decoding_options)?;
        let add_diagnostic_infos = <Option<
            Vec<crate::types::diagnostic_info::DiagnosticInfo>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::diagnostic_info::DiagnosticInfo>>,
        >>::decode(stream, decoding_options)?;
        let remove_results = <Option<
            Vec<crate::types::status_code::StatusCode>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::status_code::StatusCode>>,
        >>::decode(stream, decoding_options)?;
        let remove_diagnostic_infos = <Option<
            Vec<crate::types::diagnostic_info::DiagnosticInfo>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::diagnostic_info::DiagnosticInfo>>,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            response_header,
            add_results,
            add_diagnostic_infos,
            remove_results,
            remove_diagnostic_infos,
        })
    }
}
