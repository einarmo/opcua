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
pub struct PublishedDataSetDataType {
    pub name: opcua::types::string::UAString,
    pub data_set_folder: Option<Vec<opcua::types::string::UAString>>,
    pub data_set_meta_data: super::data_set_meta_data_type::DataSetMetaDataType,
    pub extension_fields: Option<Vec<super::key_value_pair::KeyValuePair>>,
    pub data_set_source: opcua::types::extension_object::ExtensionObject,
}
impl opcua::types::MessageInfo for PublishedDataSetDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishedDataSetDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishedDataSetDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishedDataSetDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for PublishedDataSetDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.name.byte_len();
        size += self.data_set_folder.byte_len();
        size += self.data_set_meta_data.byte_len();
        size += self.extension_fields.byte_len();
        size += self.data_set_source.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.name.encode(stream)?;
        size += self.data_set_folder.encode(stream)?;
        size += self.data_set_meta_data.encode(stream)?;
        size += self.extension_fields.encode(stream)?;
        size += self.data_set_source.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for PublishedDataSetDataType {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            name: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            data_set_folder: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            data_set_meta_data: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            extension_fields: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            data_set_source: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
