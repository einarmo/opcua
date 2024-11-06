// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
pub struct MonitoredItemCreateRequest {
    pub item_to_monitor: super::read_value_id::ReadValueId,
    pub monitoring_mode: super::enums::MonitoringMode,
    pub requested_parameters: super::monitoring_parameters::MonitoringParameters,
}
impl opcua::types::MessageInfo for MonitoredItemCreateRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::MonitoredItemCreateRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::MonitoredItemCreateRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::MonitoredItemCreateRequest_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for MonitoredItemCreateRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.item_to_monitor.byte_len();
        size += self.monitoring_mode.byte_len();
        size += self.requested_parameters.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.item_to_monitor.encode(stream)?;
        size += self.monitoring_mode.encode(stream)?;
        size += self.requested_parameters.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let item_to_monitor = <super::read_value_id::ReadValueId as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let monitoring_mode = <super::enums::MonitoringMode as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let requested_parameters = <super::monitoring_parameters::MonitoringParameters as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            item_to_monitor,
            monitoring_mode,
            requested_parameters,
        })
    }
}
