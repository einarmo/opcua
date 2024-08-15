// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransferResult {
    pub status_code: crate::types::status_code::StatusCode,
    pub available_sequence_numbers: Option<Vec<u32>>,
}
impl crate::types::MessageInfo for TransferResult {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::TransferResult_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<TransferResult> for TransferResult {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.status_code.byte_len();
        size += self.available_sequence_numbers.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.status_code.encode(stream)?;
        size += self.available_sequence_numbers.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let status_code = <crate::types::status_code::StatusCode as crate::types::BinaryEncoder<
            crate::types::status_code::StatusCode,
        >>::decode(stream, decoding_options)?;
        let available_sequence_numbers = <Option<Vec<u32>> as crate::types::BinaryEncoder<
            Option<Vec<u32>>,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            status_code,
            available_sequence_numbers,
        })
    }
}
