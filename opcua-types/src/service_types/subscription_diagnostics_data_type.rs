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
pub struct SubscriptionDiagnosticsDataType {
    pub session_id: opcua::types::node_id::NodeId,
    pub subscription_id: u32,
    pub priority: u8,
    pub publishing_interval: f64,
    pub max_keep_alive_count: u32,
    pub max_lifetime_count: u32,
    pub max_notifications_per_publish: u32,
    pub publishing_enabled: bool,
    pub modify_count: u32,
    pub enable_count: u32,
    pub disable_count: u32,
    pub republish_request_count: u32,
    pub republish_message_request_count: u32,
    pub republish_message_count: u32,
    pub transfer_request_count: u32,
    pub transferred_to_alt_client_count: u32,
    pub transferred_to_same_client_count: u32,
    pub publish_request_count: u32,
    pub data_change_notifications_count: u32,
    pub event_notifications_count: u32,
    pub notifications_count: u32,
    pub late_publish_request_count: u32,
    pub current_keep_alive_count: u32,
    pub current_lifetime_count: u32,
    pub unacknowledged_message_count: u32,
    pub discarded_message_count: u32,
    pub monitored_item_count: u32,
    pub disabled_monitored_item_count: u32,
    pub monitoring_queue_overflow_count: u32,
    pub next_sequence_number: u32,
    pub event_queue_over_flow_count: u32,
}
impl opcua::types::MessageInfo for SubscriptionDiagnosticsDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SubscriptionDiagnosticsDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SubscriptionDiagnosticsDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SubscriptionDiagnosticsDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncoder for SubscriptionDiagnosticsDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.session_id.byte_len();
        size += self.subscription_id.byte_len();
        size += self.priority.byte_len();
        size += self.publishing_interval.byte_len();
        size += self.max_keep_alive_count.byte_len();
        size += self.max_lifetime_count.byte_len();
        size += self.max_notifications_per_publish.byte_len();
        size += self.publishing_enabled.byte_len();
        size += self.modify_count.byte_len();
        size += self.enable_count.byte_len();
        size += self.disable_count.byte_len();
        size += self.republish_request_count.byte_len();
        size += self.republish_message_request_count.byte_len();
        size += self.republish_message_count.byte_len();
        size += self.transfer_request_count.byte_len();
        size += self.transferred_to_alt_client_count.byte_len();
        size += self.transferred_to_same_client_count.byte_len();
        size += self.publish_request_count.byte_len();
        size += self.data_change_notifications_count.byte_len();
        size += self.event_notifications_count.byte_len();
        size += self.notifications_count.byte_len();
        size += self.late_publish_request_count.byte_len();
        size += self.current_keep_alive_count.byte_len();
        size += self.current_lifetime_count.byte_len();
        size += self.unacknowledged_message_count.byte_len();
        size += self.discarded_message_count.byte_len();
        size += self.monitored_item_count.byte_len();
        size += self.disabled_monitored_item_count.byte_len();
        size += self.monitoring_queue_overflow_count.byte_len();
        size += self.next_sequence_number.byte_len();
        size += self.event_queue_over_flow_count.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.session_id.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += self.priority.encode(stream)?;
        size += self.publishing_interval.encode(stream)?;
        size += self.max_keep_alive_count.encode(stream)?;
        size += self.max_lifetime_count.encode(stream)?;
        size += self.max_notifications_per_publish.encode(stream)?;
        size += self.publishing_enabled.encode(stream)?;
        size += self.modify_count.encode(stream)?;
        size += self.enable_count.encode(stream)?;
        size += self.disable_count.encode(stream)?;
        size += self.republish_request_count.encode(stream)?;
        size += self.republish_message_request_count.encode(stream)?;
        size += self.republish_message_count.encode(stream)?;
        size += self.transfer_request_count.encode(stream)?;
        size += self.transferred_to_alt_client_count.encode(stream)?;
        size += self.transferred_to_same_client_count.encode(stream)?;
        size += self.publish_request_count.encode(stream)?;
        size += self.data_change_notifications_count.encode(stream)?;
        size += self.event_notifications_count.encode(stream)?;
        size += self.notifications_count.encode(stream)?;
        size += self.late_publish_request_count.encode(stream)?;
        size += self.current_keep_alive_count.encode(stream)?;
        size += self.current_lifetime_count.encode(stream)?;
        size += self.unacknowledged_message_count.encode(stream)?;
        size += self.discarded_message_count.encode(stream)?;
        size += self.monitored_item_count.encode(stream)?;
        size += self.disabled_monitored_item_count.encode(stream)?;
        size += self.monitoring_queue_overflow_count.encode(stream)?;
        size += self.next_sequence_number.encode(stream)?;
        size += self.event_queue_over_flow_count.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let session_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let subscription_id = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let priority = <u8 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let publishing_interval = <f64 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let max_keep_alive_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let max_lifetime_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let max_notifications_per_publish = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let publishing_enabled = <bool as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let modify_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let enable_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let disable_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let republish_request_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let republish_message_request_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let republish_message_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let transfer_request_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let transferred_to_alt_client_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let transferred_to_same_client_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let publish_request_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let data_change_notifications_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let event_notifications_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let notifications_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let late_publish_request_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let current_keep_alive_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let current_lifetime_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let unacknowledged_message_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let discarded_message_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let monitored_item_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let disabled_monitored_item_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let monitoring_queue_overflow_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let next_sequence_number = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let event_queue_over_flow_count = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            session_id,
            subscription_id,
            priority,
            publishing_interval,
            max_keep_alive_count,
            max_lifetime_count,
            max_notifications_per_publish,
            publishing_enabled,
            modify_count,
            enable_count,
            disable_count,
            republish_request_count,
            republish_message_request_count,
            republish_message_count,
            transfer_request_count,
            transferred_to_alt_client_count,
            transferred_to_same_client_count,
            publish_request_count,
            data_change_notifications_count,
            event_notifications_count,
            notifications_count,
            late_publish_request_count,
            current_keep_alive_count,
            current_lifetime_count,
            unacknowledged_message_count,
            discarded_message_count,
            monitored_item_count,
            disabled_monitored_item_count,
            monitoring_queue_overflow_count,
            next_sequence_number,
            event_queue_over_flow_count,
        })
    }
}
