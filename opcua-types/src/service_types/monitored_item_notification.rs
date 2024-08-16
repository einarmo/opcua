// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub struct MonitoredItemNotification {
    pub client_handle: u32,
    pub value: opcua::types::data_value::DataValue,
}
impl opcua::types::MessageInfo for MonitoredItemNotification {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::MonitoredItemNotification_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for MonitoredItemNotification {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.client_handle.byte_len();
        size += self.value.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.client_handle.encode(stream)?;
        size += self.value.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let client_handle = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let value = <opcua::types::data_value::DataValue as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self { client_handle, value })
    }
}
