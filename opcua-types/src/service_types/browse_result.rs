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
pub struct BrowseResult {
    pub status_code: opcua::types::status_code::StatusCode,
    pub continuation_point: opcua::types::byte_string::ByteString,
    pub references: Option<Vec<super::reference_description::ReferenceDescription>>,
}
impl opcua::types::MessageInfo for BrowseResult {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowseResult_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowseResult_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowseResult_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncoder for BrowseResult {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.status_code.byte_len();
        size += self.continuation_point.byte_len();
        size += self.references.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.status_code.encode(stream)?;
        size += self.continuation_point.encode(stream)?;
        size += self.references.encode(stream)?;
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
        let continuation_point = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let references = <Option<
            Vec<super::reference_description::ReferenceDescription>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            status_code,
            continuation_point,
            references,
        })
    }
}
