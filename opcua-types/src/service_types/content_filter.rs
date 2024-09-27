// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
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
#[derive(Default)]
pub struct ContentFilter {
    pub elements: Option<Vec<super::content_filter_element::ContentFilterElement>>,
}
impl opcua::types::MessageInfo for ContentFilter {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ContentFilter_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for ContentFilter {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.elements.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.elements.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let elements = <Option<
            Vec<super::content_filter_element::ContentFilterElement>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self { elements })
    }
}
