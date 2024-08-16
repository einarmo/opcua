// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct BrowsePathResult {
    pub status_code: opcua::types::status_code::StatusCode,
    pub targets: Option<Vec<super::browse_path_target::BrowsePathTarget>>,
}
impl opcua::types::MessageInfo for BrowsePathResult {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowsePathResult_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for BrowsePathResult {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.status_code.byte_len();
        size += self.targets.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.status_code.encode(stream)?;
        size += self.targets.encode(stream)?;
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
        let targets = <Option<
            Vec<super::browse_path_target::BrowsePathTarget>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self { status_code, targets })
    }
}
