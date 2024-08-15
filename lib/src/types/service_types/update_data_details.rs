// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateDataDetails {
    pub node_id: crate::types::node_id::NodeId,
    pub perform_insert_replace: super::enums::PerformUpdateType,
    pub update_values: Option<Vec<crate::types::data_value::DataValue>>,
}
impl crate::types::BinaryEncoder for UpdateDataDetails {
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
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.node_id.encode(stream)?;
        size += self.perform_insert_replace.encode(stream)?;
        size += self.update_values.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let node_id = <crate::types::node_id::NodeId as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let perform_insert_replace = <super::enums::PerformUpdateType as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let update_values = <Option<
            Vec<crate::types::data_value::DataValue>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            node_id,
            perform_insert_replace,
            update_values,
        })
    }
}
