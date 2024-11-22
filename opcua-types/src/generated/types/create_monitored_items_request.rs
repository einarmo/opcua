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
pub struct CreateMonitoredItemsRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub subscription_id: u32,
    pub timestamps_to_return: super::enums::TimestampsToReturn,
    pub items_to_create:
        Option<Vec<super::monitored_item_create_request::MonitoredItemCreateRequest>>,
}
impl opcua::types::MessageInfo for CreateMonitoredItemsRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateMonitoredItemsRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateMonitoredItemsRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateMonitoredItemsRequest_Encoding_DefaultXml
    }
}
