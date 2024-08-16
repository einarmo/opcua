// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceDescription {
    pub reference_type_id: opcua::types::node_id::NodeId,
    pub is_forward: bool,
    pub node_id: opcua::types::expanded_node_id::ExpandedNodeId,
    pub browse_name: opcua::types::qualified_name::QualifiedName,
    pub display_name: opcua::types::localized_text::LocalizedText,
    pub node_class: super::enums::NodeClass,
    pub type_definition: opcua::types::expanded_node_id::ExpandedNodeId,
}
impl opcua::types::MessageInfo for ReferenceDescription {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReferenceDescription_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for ReferenceDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.reference_type_id.byte_len();
        size += self.is_forward.byte_len();
        size += self.node_id.byte_len();
        size += self.browse_name.byte_len();
        size += self.display_name.byte_len();
        size += self.node_class.byte_len();
        size += self.type_definition.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_forward.encode(stream)?;
        size += self.node_id.encode(stream)?;
        size += self.browse_name.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.node_class.encode(stream)?;
        size += self.type_definition.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let reference_type_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let is_forward = <bool as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let node_id = <opcua::types::expanded_node_id::ExpandedNodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let browse_name = <opcua::types::qualified_name::QualifiedName as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let display_name = <opcua::types::localized_text::LocalizedText as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let node_class = <super::enums::NodeClass as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let type_definition = <opcua::types::expanded_node_id::ExpandedNodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            reference_type_id,
            is_forward,
            node_id,
            browse_name,
            display_name,
            node_class,
            type_definition,
        })
    }
}
