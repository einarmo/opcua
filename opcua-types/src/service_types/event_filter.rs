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
pub struct EventFilter {
    pub select_clauses: Option<
        Vec<super::simple_attribute_operand::SimpleAttributeOperand>,
    >,
    pub where_clause: super::content_filter::ContentFilter,
}
impl opcua::types::MessageInfo for EventFilter {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EventFilter_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EventFilter_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EventFilter_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncoder for EventFilter {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.select_clauses.byte_len();
        size += self.where_clause.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.select_clauses.encode(stream)?;
        size += self.where_clause.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let select_clauses = <Option<
            Vec<super::simple_attribute_operand::SimpleAttributeOperand>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let where_clause = <super::content_filter::ContentFilter as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            select_clauses,
            where_clause,
        })
    }
}
