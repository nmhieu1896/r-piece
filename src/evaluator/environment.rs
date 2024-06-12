use std::collections::HashMap;

use super::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment<'a> {
    pub store: HashMap<String, Object<'a>>,
}
impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: Object<'a>) {
        self.store
            .entry(key)
            .and_modify(|x| *x = value.clone())
            .or_insert(value);
    }
    pub fn get(&self, key: &str) -> Option<&Object<'a>> {
        self.store.get(key)
    }
}
