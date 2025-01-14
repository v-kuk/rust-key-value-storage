use std::{collections::HashMap, sync::{Arc, Mutex}};

pub struct KeyValueStore {
    store: Arc<Mutex<HashMap<String, String>>>
}

impl KeyValueStore {
    pub fn new () -> Self {
        KeyValueStore {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(self: &Self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let store = self.store.lock().unwrap();
        store.get(key).cloned()
    }

    pub fn del(&self, key: &str) {
        let mut store = self.store.lock().unwrap();
        store.remove(key).unwrap();
    }
}