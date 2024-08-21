// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct TransferResult {
    pub status_code: opcua::types::status_code::StatusCode,
    pub available_sequence_numbers: Option<Vec<u32>>,
}
impl opcua::types::MessageInfo for TransferResult {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::TransferResult_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for TransferResult {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.status_code.byte_len();
        size += self.available_sequence_numbers.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.status_code.encode(stream)?;
        size += self.available_sequence_numbers.encode(stream)?;
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
        let available_sequence_numbers = <Option<
            Vec<u32>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            status_code,
            available_sequence_numbers,
        })
    }
}