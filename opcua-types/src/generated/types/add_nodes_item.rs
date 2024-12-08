// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[derive(Debug, Clone, PartialEq, opcua::types::BinaryEncodable, opcua::types::BinaryDecodable)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
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
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::AddNodesItem
    }
}
