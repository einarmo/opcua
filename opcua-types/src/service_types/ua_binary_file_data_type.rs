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
pub struct UABinaryFileDataType {
    pub namespaces: Option<Vec<opcua::types::string::UAString>>,
    pub structure_data_types: Option<
        Vec<super::structure_description::StructureDescription>,
    >,
    pub enum_data_types: Option<Vec<super::enum_description::EnumDescription>>,
    pub simple_data_types: Option<
        Vec<super::simple_type_description::SimpleTypeDescription>,
    >,
    pub schema_location: opcua::types::string::UAString,
    pub file_header: Option<Vec<super::key_value_pair::KeyValuePair>>,
    pub body: opcua::types::variant::Variant,
}
impl opcua::types::MessageInfo for UABinaryFileDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UABinaryFileDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UABinaryFileDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::UABinaryFileDataType_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for UABinaryFileDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.namespaces.byte_len();
        size += self.structure_data_types.byte_len();
        size += self.enum_data_types.byte_len();
        size += self.simple_data_types.byte_len();
        size += self.schema_location.byte_len();
        size += self.file_header.byte_len();
        size += self.body.byte_len();
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
        size += self.schema_location.encode(stream)?;
        size += self.file_header.encode(stream)?;
        size += self.body.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let namespaces = <Option<
            Vec<opcua::types::string::UAString>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let structure_data_types = <Option<
            Vec<super::structure_description::StructureDescription>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let enum_data_types = <Option<
            Vec<super::enum_description::EnumDescription>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let simple_data_types = <Option<
            Vec<super::simple_type_description::SimpleTypeDescription>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let schema_location = <opcua::types::string::UAString as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let file_header = <Option<
            Vec<super::key_value_pair::KeyValuePair>,
        > as opcua::types::BinaryEncodable>::decode(stream, decoding_options)?;
        let body = <opcua::types::variant::Variant as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            namespaces,
            structure_data_types,
            enum_data_types,
            simple_data_types,
            schema_location,
            file_header,
            body,
        })
    }
}
