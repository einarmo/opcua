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
pub struct PubSubConnectionDataType {
    pub name: opcua::types::string::UAString,
    pub enabled: bool,
    pub publisher_id: opcua::types::variant::Variant,
    pub transport_profile_uri: opcua::types::string::UAString,
    pub address: opcua::types::extension_object::ExtensionObject,
    pub connection_properties: Option<Vec<super::key_value_pair::KeyValuePair>>,
    pub transport_settings: opcua::types::extension_object::ExtensionObject,
    pub writer_groups: Option<Vec<super::writer_group_data_type::WriterGroupDataType>>,
    pub reader_groups: Option<Vec<super::reader_group_data_type::ReaderGroupDataType>>,
}
impl opcua::types::MessageInfo for PubSubConnectionDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PubSubConnectionDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PubSubConnectionDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PubSubConnectionDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for PubSubConnectionDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.name.byte_len();
        size += self.enabled.byte_len();
        size += self.publisher_id.byte_len();
        size += self.transport_profile_uri.byte_len();
        size += self.address.byte_len();
        size += self.connection_properties.byte_len();
        size += self.transport_settings.byte_len();
        size += self.writer_groups.byte_len();
        size += self.reader_groups.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.name.encode(stream)?;
        size += self.enabled.encode(stream)?;
        size += self.publisher_id.encode(stream)?;
        size += self.transport_profile_uri.encode(stream)?;
        size += self.address.encode(stream)?;
        size += self.connection_properties.encode(stream)?;
        size += self.transport_settings.encode(stream)?;
        size += self.writer_groups.encode(stream)?;
        size += self.reader_groups.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let name = <opcua::types::string::UAString as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let enabled = <bool as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let publisher_id = <opcua::types::variant::Variant as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let transport_profile_uri = <opcua::types::string::UAString as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let address = <opcua::types::extension_object::ExtensionObject as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let connection_properties = <Option<
            Vec<super::key_value_pair::KeyValuePair>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let transport_settings = <opcua::types::extension_object::ExtensionObject as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let writer_groups = <Option<
            Vec<super::writer_group_data_type::WriterGroupDataType>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let reader_groups = <Option<
            Vec<super::reader_group_data_type::ReaderGroupDataType>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        Ok(Self {
            name,
            enabled,
            publisher_id,
            transport_profile_uri,
            address,
            connection_properties,
            transport_settings,
            writer_groups,
            reader_groups,
        })
    }
}
