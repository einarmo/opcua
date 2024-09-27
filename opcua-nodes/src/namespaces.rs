use std::collections::HashMap;

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
