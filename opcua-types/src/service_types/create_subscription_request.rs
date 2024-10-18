// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct CreateSubscriptionRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub requested_publishing_interval: f64,
    pub requested_lifetime_count: u32,
    pub requested_max_keep_alive_count: u32,
    pub max_notifications_per_publish: u32,
    pub publishing_enabled: bool,
    pub priority: u8,
}
impl opcua::types::MessageInfo for CreateSubscriptionRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSubscriptionRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSubscriptionRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSubscriptionRequest_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncoder for CreateSubscriptionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.requested_publishing_interval.byte_len();
        size += self.requested_lifetime_count.byte_len();
        size += self.requested_max_keep_alive_count.byte_len();
        size += self.max_notifications_per_publish.byte_len();
        size += self.publishing_enabled.byte_len();
        size += self.priority.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.requested_publishing_interval.encode(stream)?;
        size += self.requested_lifetime_count.encode(stream)?;
        size += self.requested_max_keep_alive_count.encode(stream)?;
        size += self.max_notifications_per_publish.encode(stream)?;
        size += self.publishing_enabled.encode(stream)?;
        size += self.priority.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let request_header = <opcua::types::request_header::RequestHeader as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = request_header.request_handle;
        let requested_publishing_interval = <f64 as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let requested_lifetime_count = <u32 as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let requested_max_keep_alive_count = <u32 as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let max_notifications_per_publish = <u32 as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let publishing_enabled = <bool as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let priority = <u8 as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            request_header,
            requested_publishing_interval,
            requested_lifetime_count,
            requested_max_keep_alive_count,
            max_notifications_per_publish,
            publishing_enabled,
            priority,
        })
    }
}
