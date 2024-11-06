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
pub struct AddNodesItem {
    pub parent_node_id: opcua::types::expanded_node_id::ExpandedNodeId,
    pub reference_type_id: opcua::types::node_id::NodeId,
    pub requested_new_node_id: opcua::types::expanded_node_id::ExpandedNodeId,
    pub browse_name: opcua::types::qualified_name::QualifiedName,
    pub node_class: super::enums::NodeClass,
    pub node_attributes: opcua::types::extension_object::ExtensionObject,
    pub type_definition: opcua::types::expanded_node_id::ExpandedNodeId,
}
impl opcua::types::MessageInfo for AddNodesItem {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AddNodesItem_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AddNodesItem_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AddNodesItem_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for AddNodesItem {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.parent_node_id.byte_len();
        size += self.reference_type_id.byte_len();
        size += self.requested_new_node_id.byte_len();
        size += self.browse_name.byte_len();
        size += self.node_class.byte_len();
        size += self.node_attributes.byte_len();
        size += self.type_definition.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.parent_node_id.encode(stream)?;
        size += self.reference_type_id.encode(stream)?;
        size += self.requested_new_node_id.encode(stream)?;
        size += self.browse_name.encode(stream)?;
        size += self.node_class.encode(stream)?;
        size += self.node_attributes.encode(stream)?;
        size += self.type_definition.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let parent_node_id = <opcua::types::expanded_node_id::ExpandedNodeId as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let reference_type_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let requested_new_node_id = <opcua::types::expanded_node_id::ExpandedNodeId as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let browse_name = <opcua::types::qualified_name::QualifiedName as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let node_class = <super::enums::NodeClass as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let node_attributes = <opcua::types::extension_object::ExtensionObject as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        let type_definition = <opcua::types::expanded_node_id::ExpandedNodeId as opcua::types::BinaryEncodable>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            parent_node_id,
            reference_type_id,
            requested_new_node_id,
            browse_name,
            node_class,
            node_attributes,
            type_definition,
        })
    }
}
