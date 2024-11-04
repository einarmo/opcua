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
pub struct StructureDescription {
    pub data_type_id: opcua::types::node_id::NodeId,
    pub name: opcua::types::qualified_name::QualifiedName,
    pub structure_definition: super::structure_definition::StructureDefinition,
}
impl opcua::types::MessageInfo for StructureDescription {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::StructureDescription_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::StructureDescription_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::StructureDescription_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for StructureDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.data_type_id.byte_len();
        size += self.name.byte_len();
        size += self.structure_definition.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.data_type_id.encode(stream)?;
        size += self.name.encode(stream)?;
        size += self.structure_definition.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let data_type_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let name = <opcua::types::qualified_name::QualifiedName as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let structure_definition = <super::structure_definition::StructureDefinition as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            data_type_id,
            name,
            structure_definition,
        })
    }
}
