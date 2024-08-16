// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct PublishedEventsDataType {
    pub event_notifier: opcua::types::node_id::NodeId,
    pub selected_fields: Option<
        Vec<super::simple_attribute_operand::SimpleAttributeOperand>,
    >,
    pub filter: super::content_filter::ContentFilter,
}
impl opcua::types::BinaryEncoder for PublishedEventsDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.event_notifier.byte_len();
        size += self.selected_fields.byte_len();
        size += self.filter.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.event_notifier.encode(stream)?;
        size += self.selected_fields.encode(stream)?;
        size += self.filter.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let event_notifier = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let selected_fields = <Option<
            Vec<super::simple_attribute_operand::SimpleAttributeOperand>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let filter = <super::content_filter::ContentFilter as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            event_notifier,
            selected_fields,
            filter,
        })
    }
}
