// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct SessionDiagnosticsDataType {
    pub session_id: opcua::types::node_id::NodeId,
    pub session_name: opcua::types::string::UAString,
    pub client_description: super::application_description::ApplicationDescription,
    pub server_uri: opcua::types::string::UAString,
    pub endpoint_url: opcua::types::string::UAString,
    pub locale_ids: Option<Vec<opcua::types::string::UAString>>,
    pub actual_session_timeout: f64,
    pub max_response_message_size: u32,
    pub client_connection_time: opcua::types::date_time::DateTime,
    pub client_last_contact_time: opcua::types::date_time::DateTime,
    pub current_subscriptions_count: u32,
    pub current_monitored_items_count: u32,
    pub current_publish_requests_in_queue: u32,
    pub total_request_count: super::service_counter_data_type::ServiceCounterDataType,
    pub unauthorized_request_count: u32,
    pub read_count: super::service_counter_data_type::ServiceCounterDataType,
    pub history_read_count: super::service_counter_data_type::ServiceCounterDataType,
    pub write_count: super::service_counter_data_type::ServiceCounterDataType,
    pub history_update_count: super::service_counter_data_type::ServiceCounterDataType,
    pub call_count: super::service_counter_data_type::ServiceCounterDataType,
    pub create_monitored_items_count: super::service_counter_data_type::ServiceCounterDataType,
    pub modify_monitored_items_count: super::service_counter_data_type::ServiceCounterDataType,
    pub set_monitoring_mode_count: super::service_counter_data_type::ServiceCounterDataType,
    pub set_triggering_count: super::service_counter_data_type::ServiceCounterDataType,
    pub delete_monitored_items_count: super::service_counter_data_type::ServiceCounterDataType,
    pub create_subscription_count: super::service_counter_data_type::ServiceCounterDataType,
    pub modify_subscription_count: super::service_counter_data_type::ServiceCounterDataType,
    pub set_publishing_mode_count: super::service_counter_data_type::ServiceCounterDataType,
    pub publish_count: super::service_counter_data_type::ServiceCounterDataType,
    pub republish_count: super::service_counter_data_type::ServiceCounterDataType,
    pub transfer_subscriptions_count: super::service_counter_data_type::ServiceCounterDataType,
    pub delete_subscriptions_count: super::service_counter_data_type::ServiceCounterDataType,
    pub add_nodes_count: super::service_counter_data_type::ServiceCounterDataType,
    pub add_references_count: super::service_counter_data_type::ServiceCounterDataType,
    pub delete_nodes_count: super::service_counter_data_type::ServiceCounterDataType,
    pub delete_references_count: super::service_counter_data_type::ServiceCounterDataType,
    pub browse_count: super::service_counter_data_type::ServiceCounterDataType,
    pub browse_next_count: super::service_counter_data_type::ServiceCounterDataType,
    pub translate_browse_paths_to_node_ids_count: super::service_counter_data_type::ServiceCounterDataType,
    pub query_first_count: super::service_counter_data_type::ServiceCounterDataType,
    pub query_next_count: super::service_counter_data_type::ServiceCounterDataType,
    pub register_nodes_count: super::service_counter_data_type::ServiceCounterDataType,
    pub unregister_nodes_count: super::service_counter_data_type::ServiceCounterDataType,
}
impl opcua::types::MessageInfo for SessionDiagnosticsDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionDiagnosticsDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionDiagnosticsDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionDiagnosticsDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for SessionDiagnosticsDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.session_id.byte_len();
        size += self.session_name.byte_len();
        size += self.client_description.byte_len();
        size += self.server_uri.byte_len();
        size += self.endpoint_url.byte_len();
        size += self.locale_ids.byte_len();
        size += self.actual_session_timeout.byte_len();
        size += self.max_response_message_size.byte_len();
        size += self.client_connection_time.byte_len();
        size += self.client_last_contact_time.byte_len();
        size += self.current_subscriptions_count.byte_len();
        size += self.current_monitored_items_count.byte_len();
        size += self.current_publish_requests_in_queue.byte_len();
        size += self.total_request_count.byte_len();
        size += self.unauthorized_request_count.byte_len();
        size += self.read_count.byte_len();
        size += self.history_read_count.byte_len();
        size += self.write_count.byte_len();
        size += self.history_update_count.byte_len();
        size += self.call_count.byte_len();
        size += self.create_monitored_items_count.byte_len();
        size += self.modify_monitored_items_count.byte_len();
        size += self.set_monitoring_mode_count.byte_len();
        size += self.set_triggering_count.byte_len();
        size += self.delete_monitored_items_count.byte_len();
        size += self.create_subscription_count.byte_len();
        size += self.modify_subscription_count.byte_len();
        size += self.set_publishing_mode_count.byte_len();
        size += self.publish_count.byte_len();
        size += self.republish_count.byte_len();
        size += self.transfer_subscriptions_count.byte_len();
        size += self.delete_subscriptions_count.byte_len();
        size += self.add_nodes_count.byte_len();
        size += self.add_references_count.byte_len();
        size += self.delete_nodes_count.byte_len();
        size += self.delete_references_count.byte_len();
        size += self.browse_count.byte_len();
        size += self.browse_next_count.byte_len();
        size += self.translate_browse_paths_to_node_ids_count.byte_len();
        size += self.query_first_count.byte_len();
        size += self.query_next_count.byte_len();
        size += self.register_nodes_count.byte_len();
        size += self.unregister_nodes_count.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.session_id.encode(stream)?;
        size += self.session_name.encode(stream)?;
        size += self.client_description.encode(stream)?;
        size += self.server_uri.encode(stream)?;
        size += self.endpoint_url.encode(stream)?;
        size += self.locale_ids.encode(stream)?;
        size += self.actual_session_timeout.encode(stream)?;
        size += self.max_response_message_size.encode(stream)?;
        size += self.client_connection_time.encode(stream)?;
        size += self.client_last_contact_time.encode(stream)?;
        size += self.current_subscriptions_count.encode(stream)?;
        size += self.current_monitored_items_count.encode(stream)?;
        size += self.current_publish_requests_in_queue.encode(stream)?;
        size += self.total_request_count.encode(stream)?;
        size += self.unauthorized_request_count.encode(stream)?;
        size += self.read_count.encode(stream)?;
        size += self.history_read_count.encode(stream)?;
        size += self.write_count.encode(stream)?;
        size += self.history_update_count.encode(stream)?;
        size += self.call_count.encode(stream)?;
        size += self.create_monitored_items_count.encode(stream)?;
        size += self.modify_monitored_items_count.encode(stream)?;
        size += self.set_monitoring_mode_count.encode(stream)?;
        size += self.set_triggering_count.encode(stream)?;
        size += self.delete_monitored_items_count.encode(stream)?;
        size += self.create_subscription_count.encode(stream)?;
        size += self.modify_subscription_count.encode(stream)?;
        size += self.set_publishing_mode_count.encode(stream)?;
        size += self.publish_count.encode(stream)?;
        size += self.republish_count.encode(stream)?;
        size += self.transfer_subscriptions_count.encode(stream)?;
        size += self.delete_subscriptions_count.encode(stream)?;
        size += self.add_nodes_count.encode(stream)?;
        size += self.add_references_count.encode(stream)?;
        size += self.delete_nodes_count.encode(stream)?;
        size += self.delete_references_count.encode(stream)?;
        size += self.browse_count.encode(stream)?;
        size += self.browse_next_count.encode(stream)?;
        size += self.translate_browse_paths_to_node_ids_count.encode(stream)?;
        size += self.query_first_count.encode(stream)?;
        size += self.query_next_count.encode(stream)?;
        size += self.register_nodes_count.encode(stream)?;
        size += self.unregister_nodes_count.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for SessionDiagnosticsDataType {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            session_id: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            session_name: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            client_description: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            server_uri: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            endpoint_url: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            locale_ids: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            actual_session_timeout: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            max_response_message_size: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            client_connection_time: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            client_last_contact_time: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            current_subscriptions_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            current_monitored_items_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            current_publish_requests_in_queue: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            total_request_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            unauthorized_request_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            read_count: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            history_read_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            write_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            history_update_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            call_count: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            create_monitored_items_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            modify_monitored_items_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            set_monitoring_mode_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            set_triggering_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            delete_monitored_items_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            create_subscription_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            modify_subscription_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            set_publishing_mode_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            publish_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            republish_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            transfer_subscriptions_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            delete_subscriptions_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            add_nodes_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            add_references_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            delete_nodes_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            delete_references_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            browse_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            browse_next_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            translate_browse_paths_to_node_ids_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            query_first_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            query_next_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            register_nodes_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            unregister_nodes_count: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
