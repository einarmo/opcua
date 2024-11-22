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
pub struct OpenSecureChannelResponse {
    pub response_header: opcua::types::response_header::ResponseHeader,
    pub server_protocol_version: u32,
    pub security_token: super::channel_security_token::ChannelSecurityToken,
    pub server_nonce: opcua::types::byte_string::ByteString,
}
impl opcua::types::MessageInfo for OpenSecureChannelResponse {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::OpenSecureChannelResponse_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::OpenSecureChannelResponse_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::OpenSecureChannelResponse_Encoding_DefaultXml
    }
}
