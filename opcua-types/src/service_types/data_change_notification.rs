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
#[derive(Default)]
pub struct DataChangeNotification {
    pub monitored_items: Option<
        Vec<super::monitored_item_notification::MonitoredItemNotification>,
    >,
    pub diagnostic_infos: Option<Vec<opcua::types::diagnostic_info::DiagnosticInfo>>,
}
impl opcua::types::MessageInfo for DataChangeNotification {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataChangeNotification_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataChangeNotification_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataChangeNotification_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for DataChangeNotification {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.monitored_items.byte_len();
        size += self.diagnostic_infos.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.monitored_items.encode(stream)?;
        size += self.diagnostic_infos.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let monitored_items = <Option<
            Vec<super::monitored_item_notification::MonitoredItemNotification>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let diagnostic_infos = <Option<
            Vec<opcua::types::diagnostic_info::DiagnosticInfo>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        Ok(Self {
            monitored_items,
            diagnostic_infos,
        })
    }
}
