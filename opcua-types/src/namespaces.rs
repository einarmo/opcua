use hashbrown::HashMap;

/// Utility for handling assignment of namespaces on server startup.
#[derive(Debug, Default, Clone)]
pub struct NamespaceMap {
    known_namespaces: HashMap<String, u16>,
}

impl NamespaceMap {
    pub fn new() -> Self {
        let mut known_namespaces = HashMap::new();
        known_namespaces.insert("http://opcfoundation.org/UA/".to_owned(), 0u16);

        Self { known_namespaces }
    }

    pub fn new_full(map: HashMap<String, u16>) -> Self {
        Self {
            known_namespaces: map,
        }
    }

    pub fn add_namespace(&mut self, namespace: &str) -> u16 {
        if let Some(ns) = self.known_namespaces.get(namespace) {
            return *ns;
        }
        let max = self
            .known_namespaces
            .iter()
            .map(|kv| *kv.1)
            .max()
            .unwrap_or_default();
        self.known_namespaces.insert(namespace.to_owned(), max + 1);

        max + 1
    }

    pub fn known_namespaces(&self) -> &HashMap<String, u16> {
        &self.known_namespaces
    }

    pub fn get_index(&self, ns: &str) -> Option<u16> {
        self.known_namespaces.get(ns).copied()
    }
}

/// Utility handling namespaces when loading node sets.
pub struct NodeSetNamespaceMapper<'a> {
    namespaces: &'a mut NamespaceMap,
    index_map: HashMap<u16, u16>,
}

#[derive(Debug)]
pub struct UninitializedIndex(pub u16);

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

    pub fn get_index(&self, index_in_node_set: u16) -> Result<u16, UninitializedIndex> {
        if index_in_node_set == 0 {
            return Ok(0);
        }
        let Some(idx) = self.index_map.get(&index_in_node_set) else {
            return Err(UninitializedIndex(index_in_node_set));
        };
        Ok(*idx)
    }

    pub fn namespaces(&'a self) -> &'a NamespaceMap {
        &*self.namespaces
    }
}
