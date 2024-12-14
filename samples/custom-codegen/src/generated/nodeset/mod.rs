// This file was autogenerated from schema/Opc.Ua.Pn.NodeSet2.xml by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Einar Omang
mod nodeset_1;
mod nodeset_2;
mod nodeset_3;
mod nodeset_4;
pub struct ProfinetNamespace;
impl opcua::nodes::NodeSetImport for ProfinetNamespace {
    fn load<'a>(
        &'a self,
        map: &'a opcua::nodes::NodeSetNamespaceMapper,
    ) -> Box<dyn Iterator<Item = opcua::nodes::ImportedItem> + 'a> {
        Box::new(
            [
                nodeset_1::imported_nodes(map),
                nodeset_2::imported_nodes(map),
                nodeset_3::imported_nodes(map),
                nodeset_4::imported_nodes(map),
            ]
            .into_iter()
            .flatten(),
        )
    }
    fn register_namespaces(&self, map: &mut opcua::nodes::NodeSetNamespaceMapper) {
        map.add_namespace("http://opcfoundation.org/UA/", 0u16);
        map.add_namespace("http://opcfoundation.org/UA/PROFINET/", 1u16);
    }
    fn get_own_namespaces(&self) -> Vec<String> {
        vec!["http://opcfoundation.org/UA/PROFINET/".to_owned()]
    }
}