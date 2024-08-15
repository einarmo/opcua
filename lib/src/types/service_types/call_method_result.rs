// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct CallMethodResult {
    pub status_code: crate::types::status_code::StatusCode,
    pub input_argument_results: Option<Vec<crate::types::status_code::StatusCode>>,
    pub input_argument_diagnostic_infos: Option<
        Vec<crate::types::diagnostic_info::DiagnosticInfo>,
    >,
    pub output_arguments: Option<Vec<crate::types::variant::Variant>>,
}
impl crate::types::MessageInfo for CallMethodResult {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::CallMethodResult_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for CallMethodResult {
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
    ) -> crate::types::EncodingResult<usize> {
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
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let status_code = <crate::types::status_code::StatusCode as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let input_argument_results = <Option<
            Vec<crate::types::status_code::StatusCode>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let input_argument_diagnostic_infos = <Option<
            Vec<crate::types::diagnostic_info::DiagnosticInfo>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let output_arguments = <Option<
            Vec<crate::types::variant::Variant>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            status_code,
            input_argument_results,
            input_argument_diagnostic_infos,
            output_arguments,
        })
    }
}
