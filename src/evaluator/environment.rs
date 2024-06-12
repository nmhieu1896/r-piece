use std::collections::HashMap;

use super::object::Object;

pub struct Environment {
    pub store: HashMap<String, Object>,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: Object) {
        self.store
            .entry(key)
            .and_modify(|x| *x = value.clone())
            .or_insert(value);
    }
    pub fn get(&self, key: &str) -> Option<&Object> {
        self.store.get(key)
    }
}
