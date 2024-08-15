// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PublishedDataSetDataType {
    pub name: crate::types::string::UAString,
    pub data_set_folder: Option<Vec<crate::types::string::UAString>>,
    pub data_set_meta_data: super::data_set_meta_data_type::DataSetMetaDataType,
    pub extension_fields: Option<Vec<super::key_value_pair::KeyValuePair>>,
    pub data_set_source: crate::types::extension_object::ExtensionObject,
}
impl crate::types::MessageInfo for PublishedDataSetDataType {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::PublishedDataSetDataType_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<PublishedDataSetDataType> for PublishedDataSetDataType {
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
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.name.encode(stream)?;
        size += self.data_set_folder.encode(stream)?;
        size += self.data_set_meta_data.encode(stream)?;
        size += self.extension_fields.encode(stream)?;
        size += self.data_set_source.encode(stream)?;
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
        let data_set_folder =
            <Option<Vec<crate::types::string::UAString>> as crate::types::BinaryEncoder<
                Option<Vec<crate::types::string::UAString>>,
            >>::decode(stream, decoding_options)?;
        let data_set_meta_data =
            <super::data_set_meta_data_type::DataSetMetaDataType as crate::types::BinaryEncoder<
                super::data_set_meta_data_type::DataSetMetaDataType,
            >>::decode(stream, decoding_options)?;
        let extension_fields =
            <Option<Vec<super::key_value_pair::KeyValuePair>> as crate::types::BinaryEncoder<
                Option<Vec<super::key_value_pair::KeyValuePair>>,
            >>::decode(stream, decoding_options)?;
        let data_set_source =
            <crate::types::extension_object::ExtensionObject as crate::types::BinaryEncoder<
                crate::types::extension_object::ExtensionObject,
            >>::decode(stream, decoding_options)?;
        Ok(Self {
            name,
            data_set_folder,
            data_set_meta_data,
            extension_fields,
            data_set_source,
        })
    }
}
