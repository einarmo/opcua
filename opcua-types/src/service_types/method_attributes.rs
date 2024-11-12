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
pub struct MethodAttributes {
    pub specified_attributes: u32,
    pub display_name: opcua::types::localized_text::LocalizedText,
    pub description: opcua::types::localized_text::LocalizedText,
    pub write_mask: u32,
    pub user_write_mask: u32,
    pub executable: bool,
    pub user_executable: bool,
}
impl opcua::types::MessageInfo for MethodAttributes {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::MethodAttributes_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::MethodAttributes_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::MethodAttributes_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for MethodAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += self.executable.byte_len();
        size += self.user_executable.byte_len();
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
        size += self.executable.encode(stream)?;
        size += self.user_executable.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for MethodAttributes {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            specified_attributes: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            display_name: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            description: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            write_mask: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            user_write_mask: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            executable: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            user_executable: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
