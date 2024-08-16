// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
pub struct UpdateStructureDataDetails {
    pub node_id: opcua::types::node_id::NodeId,
    pub perform_insert_replace: super::enums::PerformUpdateType,
    pub update_values: Option<Vec<opcua::types::data_value::DataValue>>,
}
impl opcua::types::BinaryEncoder for UpdateStructureDataDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.node_id.byte_len();
        size += self.perform_insert_replace.byte_len();
        size += self.update_values.byte_len();
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
        size += self.update_values.encode(stream)?;
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
        let update_values = <Option<
            Vec<opcua::types::data_value::DataValue>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            node_id,
            perform_insert_replace,
            update_values,
        })
    }
}
