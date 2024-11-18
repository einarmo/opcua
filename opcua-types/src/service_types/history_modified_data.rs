// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct HistoryModifiedData {
    pub data_values: Option<Vec<opcua::types::data_value::DataValue>>,
    pub modification_infos: Option<Vec<super::modification_info::ModificationInfo>>,
}
impl opcua::types::MessageInfo for HistoryModifiedData {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::HistoryModifiedData_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::HistoryModifiedData_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::HistoryModifiedData_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for HistoryModifiedData {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.data_values.byte_len();
        size += self.modification_infos.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.data_values.encode(stream)?;
        size += self.modification_infos.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for HistoryModifiedData {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            data_values: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            modification_infos: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
