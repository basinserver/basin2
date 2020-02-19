use std::collections::HashMap;
use std::sync::Arc;
use std::hash::Hash;

pub trait RegistryItem: Send + Sync {

}

// we don't use a linked hashmap here due to indexing requirements
pub struct Registry<T: RegistryItem> {
    indexed: Vec<Arc<T>>,
    name_index_map: HashMap<String, usize>,
}

fn namespace_normalize(key: &str) -> String {
    let split_index = key.find(":");
    let (namespace, name) = match split_index {
        Some(split_index) => key.split_at(split_index),
        None => ("minecraft", key),
    };
    format!("{}:{}", namespace, name)
}

impl<T: RegistryItem> Registry<T> {

    pub fn new() -> Registry<T> {
        return Registry {
            indexed: vec![],
            name_index_map: HashMap::new(),
        };
    }

    pub fn insert(&mut self, key: &str, item: T) {
        let key = namespace_normalize(key);
        let item = Arc::new(item);
        let index = self.indexed.len();
        self.name_index_map.insert(key, index);
        self.indexed.push(item);
    }

    fn get(&self, index: usize) -> Option<Arc<T>> {
        self.indexed.get(index).map(|arc| arc.clone())
    }

    fn get_str(&self, key: &str) -> Option<Arc<T>> {
        let key = namespace_normalize(key);
        return self.name_index_map.get(&key).map(|index| self.indexed[*index].clone());
    }

}
