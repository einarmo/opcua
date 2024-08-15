// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousIdentityToken {
    pub policy_id: crate::types::string::UAString,
}
impl crate::types::BinaryEncoder<AnonymousIdentityToken> for AnonymousIdentityToken {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.policy_id.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.policy_id.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let policy_id = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        Ok(Self { policy_id })
    }
}
