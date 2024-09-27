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
pub struct ReadAnnotationDataDetails {
    pub req_times: Option<Vec<opcua::types::date_time::DateTime>>,
}
impl opcua::types::MessageInfo for ReadAnnotationDataDetails {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReadAnnotationDataDetails_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for ReadAnnotationDataDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.req_times.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.req_times.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let req_times = <Option<
            Vec<opcua::types::date_time::DateTime>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self { req_times })
    }
}
