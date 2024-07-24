// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct WriterGroupDataType {
    pub name: crate::types::string::UAString,
    pub enabled: bool,
    pub security_mode: super::enums::MessageSecurityMode,
    pub security_group_id: crate::types::string::UAString,
    pub security_key_services: Option<
        Vec<super::endpoint_description::EndpointDescription>,
    >,
    pub max_network_message_size: u32,
    pub group_properties: Option<Vec<super::key_value_pair::KeyValuePair>>,
    pub writer_group_id: u16,
    pub publishing_interval: f64,
    pub keep_alive_time: f64,
    pub priority: u8,
    pub locale_ids: Option<Vec<crate::types::string::UAString>>,
    pub header_layout_uri: crate::types::string::UAString,
    pub transport_settings: crate::types::extension_object::ExtensionObject,
    pub message_settings: crate::types::extension_object::ExtensionObject,
    pub data_set_writers: Option<
        Vec<super::data_set_writer_data_type::DataSetWriterDataType>,
    >,
}
impl crate::types::BinaryEncoder<WriterGroupDataType> for WriterGroupDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.name.byte_len();
        size += self.enabled.byte_len();
        size += self.security_mode.byte_len();
        size += self.security_group_id.byte_len();
        size += self.security_key_services.byte_len();
        size += self.max_network_message_size.byte_len();
        size += self.group_properties.byte_len();
        size += self.writer_group_id.byte_len();
        size += self.publishing_interval.byte_len();
        size += self.keep_alive_time.byte_len();
        size += self.priority.byte_len();
        size += self.locale_ids.byte_len();
        size += self.header_layout_uri.byte_len();
        size += self.transport_settings.byte_len();
        size += self.message_settings.byte_len();
        size += self.data_set_writers.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.name.encode(stream)?;
        size += self.enabled.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.security_group_id.encode(stream)?;
        size += self.security_key_services.encode(stream)?;
        size += self.max_network_message_size.encode(stream)?;
        size += self.group_properties.encode(stream)?;
        size += self.writer_group_id.encode(stream)?;
        size += self.publishing_interval.encode(stream)?;
        size += self.keep_alive_time.encode(stream)?;
        size += self.priority.encode(stream)?;
        size += self.locale_ids.encode(stream)?;
        size += self.header_layout_uri.encode(stream)?;
        size += self.transport_settings.encode(stream)?;
        size += self.message_settings.encode(stream)?;
        size += self.data_set_writers.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let name = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let enabled = <bool as crate::types::BinaryEncoder<
            bool,
        >>::decode(stream, decoding_options)?;
        let security_mode = <super::enums::MessageSecurityMode as crate::types::BinaryEncoder<
            super::enums::MessageSecurityMode,
        >>::decode(stream, decoding_options)?;
        let security_group_id = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let security_key_services = <Option<
            Vec<super::endpoint_description::EndpointDescription>,
        > as crate::types::BinaryEncoder<
            Option<Vec<super::endpoint_description::EndpointDescription>>,
        >>::decode(stream, decoding_options)?;
        let max_network_message_size = <u32 as crate::types::BinaryEncoder<
            u32,
        >>::decode(stream, decoding_options)?;
        let group_properties = <Option<
            Vec<super::key_value_pair::KeyValuePair>,
        > as crate::types::BinaryEncoder<
            Option<Vec<super::key_value_pair::KeyValuePair>>,
        >>::decode(stream, decoding_options)?;
        let writer_group_id = <u16 as crate::types::BinaryEncoder<
            u16,
        >>::decode(stream, decoding_options)?;
        let publishing_interval = <f64 as crate::types::BinaryEncoder<
            f64,
        >>::decode(stream, decoding_options)?;
        let keep_alive_time = <f64 as crate::types::BinaryEncoder<
            f64,
        >>::decode(stream, decoding_options)?;
        let priority = <u8 as crate::types::BinaryEncoder<
            u8,
        >>::decode(stream, decoding_options)?;
        let locale_ids = <Option<
            Vec<crate::types::string::UAString>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::string::UAString>>,
        >>::decode(stream, decoding_options)?;
        let header_layout_uri = <crate::types::string::UAString as crate::types::BinaryEncoder<
            crate::types::string::UAString,
        >>::decode(stream, decoding_options)?;
        let transport_settings = <crate::types::extension_object::ExtensionObject as crate::types::BinaryEncoder<
            crate::types::extension_object::ExtensionObject,
        >>::decode(stream, decoding_options)?;
        let message_settings = <crate::types::extension_object::ExtensionObject as crate::types::BinaryEncoder<
            crate::types::extension_object::ExtensionObject,
        >>::decode(stream, decoding_options)?;
        let data_set_writers = <Option<
            Vec<super::data_set_writer_data_type::DataSetWriterDataType>,
        > as crate::types::BinaryEncoder<
            Option<Vec<super::data_set_writer_data_type::DataSetWriterDataType>>,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            name,
            enabled,
            security_mode,
            security_group_id,
            security_key_services,
            max_network_message_size,
            group_properties,
            writer_group_id,
            publishing_interval,
            keep_alive_time,
            priority,
            locale_ids,
            header_layout_uri,
            transport_settings,
            message_settings,
            data_set_writers,
        })
    }
}
