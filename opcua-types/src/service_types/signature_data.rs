// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct SignatureData {
    pub algorithm: opcua::types::string::UAString,
    pub signature: opcua::types::byte_string::ByteString,
}
impl opcua::types::MessageInfo for SignatureData {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SignatureData_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for SignatureData {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.algorithm.byte_len();
        size += self.signature.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.algorithm.encode(stream)?;
        size += self.signature.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let algorithm = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let signature = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self { algorithm, signature })
    }
}