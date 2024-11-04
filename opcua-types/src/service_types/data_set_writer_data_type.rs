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
pub struct DataSetWriterDataType {
    pub name: opcua::types::string::UAString,
    pub enabled: bool,
    pub data_set_writer_id: u16,
    pub data_set_field_content_mask: super::enums::DataSetFieldContentMask,
    pub key_frame_count: u32,
    pub data_set_name: opcua::types::string::UAString,
    pub data_set_writer_properties: Option<Vec<super::key_value_pair::KeyValuePair>>,
    pub transport_settings: opcua::types::extension_object::ExtensionObject,
    pub message_settings: opcua::types::extension_object::ExtensionObject,
}
impl opcua::types::MessageInfo for DataSetWriterDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataSetWriterDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataSetWriterDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataSetWriterDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for DataSetWriterDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.name.byte_len();
        size += self.enabled.byte_len();
        size += self.data_set_writer_id.byte_len();
        size += self.data_set_field_content_mask.byte_len();
        size += self.key_frame_count.byte_len();
        size += self.data_set_name.byte_len();
        size += self.data_set_writer_properties.byte_len();
        size += self.transport_settings.byte_len();
        size += self.message_settings.byte_len();
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
        size += self.data_set_writer_id.encode(stream)?;
        size += self.data_set_field_content_mask.encode(stream)?;
        size += self.key_frame_count.encode(stream)?;
        size += self.data_set_name.encode(stream)?;
        size += self.data_set_writer_properties.encode(stream)?;
        size += self.transport_settings.encode(stream)?;
        size += self.message_settings.encode(stream)?;
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
        let data_set_writer_id = <u16 as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let data_set_field_content_mask = <super::enums::DataSetFieldContentMask as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let key_frame_count = <u32 as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let data_set_name = <opcua::types::string::UAString as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let data_set_writer_properties = <Option<
            Vec<super::key_value_pair::KeyValuePair>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let transport_settings = <opcua::types::extension_object::ExtensionObject as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let message_settings = <opcua::types::extension_object::ExtensionObject as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            name,
            enabled,
            data_set_writer_id,
            data_set_field_content_mask,
            key_frame_count,
            data_set_name,
            data_set_writer_properties,
            transport_settings,
            message_settings,
        })
    }
}
