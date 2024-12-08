// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[derive(Debug, Clone, PartialEq, opcua::types::BinaryEncodable, opcua::types::BinaryDecodable)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct DataChangeNotification {
    pub monitored_items: Option<Vec<super::monitored_item_notification::MonitoredItemNotification>>,
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
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::DataChangeNotification
    }
}
