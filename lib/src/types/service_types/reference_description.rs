// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceDescription {
    pub reference_type_id: crate::types::node_id::NodeId,
    pub is_forward: bool,
    pub node_id: crate::types::expanded_node_id::ExpandedNodeId,
    pub browse_name: crate::types::qualified_name::QualifiedName,
    pub display_name: crate::types::localized_text::LocalizedText,
    pub node_class: super::enums::NodeClass,
    pub type_definition: crate::types::expanded_node_id::ExpandedNodeId,
}
impl crate::types::MessageInfo for ReferenceDescription {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::ReferenceDescription_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<ReferenceDescription> for ReferenceDescription {
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
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
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
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let reference_type_id = <crate::types::node_id::NodeId as crate::types::BinaryEncoder<
            crate::types::node_id::NodeId,
        >>::decode(stream, decoding_options)?;
        let is_forward =
            <bool as crate::types::BinaryEncoder<bool>>::decode(stream, decoding_options)?;
        let node_id =
            <crate::types::expanded_node_id::ExpandedNodeId as crate::types::BinaryEncoder<
                crate::types::expanded_node_id::ExpandedNodeId,
            >>::decode(stream, decoding_options)?;
        let browse_name =
            <crate::types::qualified_name::QualifiedName as crate::types::BinaryEncoder<
                crate::types::qualified_name::QualifiedName,
            >>::decode(stream, decoding_options)?;
        let display_name =
            <crate::types::localized_text::LocalizedText as crate::types::BinaryEncoder<
                crate::types::localized_text::LocalizedText,
            >>::decode(stream, decoding_options)?;
        let node_class = <super::enums::NodeClass as crate::types::BinaryEncoder<
            super::enums::NodeClass,
        >>::decode(stream, decoding_options)?;
        let type_definition =
            <crate::types::expanded_node_id::ExpandedNodeId as crate::types::BinaryEncoder<
                crate::types::expanded_node_id::ExpandedNodeId,
            >>::decode(stream, decoding_options)?;
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
