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
pub struct ObjectTypeAttributes {
    pub specified_attributes: u32,
    pub display_name: opcua::types::localized_text::LocalizedText,
    pub description: opcua::types::localized_text::LocalizedText,
    pub write_mask: u32,
    pub user_write_mask: u32,
    pub is_abstract: bool,
}
impl opcua::types::MessageInfo for ObjectTypeAttributes {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ObjectTypeAttributes_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ObjectTypeAttributes_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ObjectTypeAttributes_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for ObjectTypeAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += self.is_abstract.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.specified_attributes.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += self.is_abstract.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let specified_attributes = <u32 as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let display_name = <opcua::types::localized_text::LocalizedText as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let description = <opcua::types::localized_text::LocalizedText as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let write_mask = <u32 as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let user_write_mask = <u32 as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let is_abstract = <bool as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            specified_attributes,
            display_name,
            description,
            write_mask,
            user_write_mask,
            is_abstract,
        })
    }
}
