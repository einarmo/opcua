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
pub struct DataSetMetaDataType {
    pub namespaces: Option<Vec<opcua::types::string::UAString>>,
    pub structure_data_types: Option<
        Vec<super::structure_description::StructureDescription>,
    >,
    pub enum_data_types: Option<Vec<super::enum_description::EnumDescription>>,
    pub simple_data_types: Option<
        Vec<super::simple_type_description::SimpleTypeDescription>,
    >,
    pub name: opcua::types::string::UAString,
    pub description: opcua::types::localized_text::LocalizedText,
    pub fields: Option<Vec<super::field_meta_data::FieldMetaData>>,
    pub data_set_class_id: opcua::types::guid::Guid,
    pub configuration_version: super::configuration_version_data_type::ConfigurationVersionDataType,
}
impl opcua::types::MessageInfo for DataSetMetaDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataSetMetaDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataSetMetaDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataSetMetaDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for DataSetMetaDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.namespaces.byte_len();
        size += self.structure_data_types.byte_len();
        size += self.enum_data_types.byte_len();
        size += self.simple_data_types.byte_len();
        size += self.name.byte_len();
        size += self.description.byte_len();
        size += self.fields.byte_len();
        size += self.data_set_class_id.byte_len();
        size += self.configuration_version.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.namespaces.encode(stream)?;
        size += self.structure_data_types.encode(stream)?;
        size += self.enum_data_types.encode(stream)?;
        size += self.simple_data_types.encode(stream)?;
        size += self.name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.fields.encode(stream)?;
        size += self.data_set_class_id.encode(stream)?;
        size += self.configuration_version.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            namespaces: opcua::types::BinaryEncodable::decode(stream, decoding_options)?,
            structure_data_types: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            enum_data_types: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            simple_data_types: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            name: opcua::types::BinaryEncodable::decode(stream, decoding_options)?,
            description: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            fields: opcua::types::BinaryEncodable::decode(stream, decoding_options)?,
            data_set_class_id: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            configuration_version: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
