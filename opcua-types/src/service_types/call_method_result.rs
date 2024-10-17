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
pub struct CallMethodResult {
    pub status_code: opcua::types::status_code::StatusCode,
    pub input_argument_results: Option<Vec<opcua::types::status_code::StatusCode>>,
    pub input_argument_diagnostic_infos: Option<
        Vec<opcua::types::diagnostic_info::DiagnosticInfo>,
    >,
    pub output_arguments: Option<Vec<opcua::types::variant::Variant>>,
}
impl opcua::types::MessageInfo for CallMethodResult {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CallMethodResult_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for CallMethodResult {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.status_code.byte_len();
        size += self.input_argument_results.byte_len();
        size += self.input_argument_diagnostic_infos.byte_len();
        size += self.output_arguments.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.status_code.encode(stream)?;
        size += self.input_argument_results.encode(stream)?;
        size += self.input_argument_diagnostic_infos.encode(stream)?;
        size += self.output_arguments.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let status_code = <opcua::types::status_code::StatusCode as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let input_argument_results = <Option<
            Vec<opcua::types::status_code::StatusCode>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let input_argument_diagnostic_infos = <Option<
            Vec<opcua::types::diagnostic_info::DiagnosticInfo>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let output_arguments = <Option<
            Vec<opcua::types::variant::Variant>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            status_code,
            input_argument_results,
            input_argument_diagnostic_infos,
            output_arguments,
        })
    }
}
