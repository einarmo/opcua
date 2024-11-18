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
pub struct EnumDescription {
    pub data_type_id: opcua::types::node_id::NodeId,
    pub name: opcua::types::qualified_name::QualifiedName,
    pub enum_definition: super::enum_definition::EnumDefinition,
    pub built_in_type: u8,
}
impl opcua::types::MessageInfo for EnumDescription {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EnumDescription_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EnumDescription_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EnumDescription_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for EnumDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.data_type_id.byte_len();
        size += self.name.byte_len();
        size += self.enum_definition.byte_len();
        size += self.built_in_type.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.data_type_id.encode(stream)?;
        size += self.name.encode(stream)?;
        size += self.enum_definition.encode(stream)?;
        size += self.built_in_type.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for EnumDescription {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            data_type_id: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            name: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            enum_definition: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            built_in_type: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
        })
    }
}
