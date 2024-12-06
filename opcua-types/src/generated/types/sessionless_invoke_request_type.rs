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
pub struct SessionlessInvokeRequestType {
    pub uris_version: u32,
    pub namespace_uris: Option<Vec<opcua::types::string::UAString>>,
    pub server_uris: Option<Vec<opcua::types::string::UAString>>,
    pub locale_ids: Option<Vec<opcua::types::string::UAString>>,
    pub service_id: u32,
}
impl opcua::types::MessageInfo for SessionlessInvokeRequestType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionlessInvokeRequestType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionlessInvokeRequestType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionlessInvokeRequestType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::SessionlessInvokeRequestType
    }
}
