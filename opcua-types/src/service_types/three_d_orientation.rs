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
pub struct ThreeDOrientation {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
impl opcua::types::MessageInfo for ThreeDOrientation {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ThreeDOrientation_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for ThreeDOrientation {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.a.byte_len();
        size += self.b.byte_len();
        size += self.c.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.a.encode(stream)?;
        size += self.b.encode(stream)?;
        size += self.c.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let a = <f64 as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let b = <f64 as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let c = <f64 as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self { a, b, c })
    }
}
