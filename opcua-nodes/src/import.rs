use std::collections::HashMap;

use opcua_types::NodeId;

use crate::NamespaceMap;

use super::NodeType;

/// Utility handling namespaces when loading node sets.
pub struct NodeSetNamespaceMapper<'a> {
    namespaces: &'a mut NamespaceMap,
    index_map: HashMap<u16, u16>,
}

impl<'a> NodeSetNamespaceMapper<'a> {
    pub fn new(namespaces: &'a mut NamespaceMap) -> Self {
        Self {
            namespaces,
            index_map: HashMap::new(),
        }
    }

    pub fn add_namespace(&mut self, namespace: &str, index_in_node_set: u16) {
        let index = self.namespaces.add_namespace(namespace);
        self.index_map.insert(index_in_node_set, index);
    }

    pub fn get_index(&self, index_in_node_set: u16) -> u16 {
        if index_in_node_set == 0 {
            return 0;
        }
        let Some(idx) = self.index_map.get(&index_in_node_set) else {
            panic!("Requested unitialized index: {index_in_node_set}");
        };
        *idx
    }

    pub fn namespaces(&'a self) -> &'a NamespaceMap {
        &*self.namespaces
    }
}

#[derive(Debug)]
pub struct ImportedReference {
    pub target_id: NodeId,
    pub type_id: NodeId,
    pub is_forward: bool,
}

#[derive(Debug)]
pub struct ImportedItem {
    pub node: NodeType,
    pub references: Vec<ImportedReference>,
}

pub trait NodeSetImport {
    fn register_namespaces(namespaces: &mut NodeSetNamespaceMapper) -> Vec<String>;

    fn load<'a>(namespaces: &'a NodeSetNamespaceMapper) -> impl Iterator<Item = ImportedItem> + 'a;
}
