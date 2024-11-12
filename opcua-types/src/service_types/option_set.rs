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
pub struct OptionSet {
    pub value: opcua::types::byte_string::ByteString,
    pub valid_bits: opcua::types::byte_string::ByteString,
}
impl opcua::types::MessageInfo for OptionSet {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::OptionSet_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::OptionSet_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::OptionSet_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for OptionSet {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.value.byte_len();
        size += self.valid_bits.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.value.encode(stream)?;
        size += self.valid_bits.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for OptionSet {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            value: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            valid_bits: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
        })
    }
}
