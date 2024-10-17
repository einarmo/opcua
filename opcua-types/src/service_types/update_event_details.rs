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
pub struct UpdateEventDetails {
    pub node_id: opcua::types::node_id::NodeId,
    pub perform_insert_replace: super::enums::PerformUpdateType,
    pub filter: super::event_filter::EventFilter,
    pub event_data: Option<Vec<super::history_event_field_list::HistoryEventFieldList>>,
}
impl opcua::types::MessageInfo for UpdateEventDetails {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UpdateEventDetails_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for UpdateEventDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.node_id.byte_len();
        size += self.perform_insert_replace.byte_len();
        size += self.filter.byte_len();
        size += self.event_data.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.node_id.encode(stream)?;
        size += self.perform_insert_replace.encode(stream)?;
        size += self.filter.encode(stream)?;
        size += self.event_data.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let node_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let perform_insert_replace = <super::enums::PerformUpdateType as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let filter = <super::event_filter::EventFilter as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let event_data = <Option<
            Vec<super::history_event_field_list::HistoryEventFieldList>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            node_id,
            perform_insert_replace,
            filter,
            event_data,
        })
    }
}
