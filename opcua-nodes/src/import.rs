use opcua_types::NodeId;

use super::NodeType;

pub use opcua_types::NodeSetNamespaceMapper;

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
    fn register_namespaces(&self, namespaces: &mut NodeSetNamespaceMapper) -> Vec<String>;

    fn load<'a>(
        &'a self,
        namespaces: &'a NodeSetNamespaceMapper,
    ) -> impl Iterator<Item = ImportedItem> + 'a;
}
