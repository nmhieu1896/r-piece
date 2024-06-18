use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::errors::eval_errs::EvalErr;

use super::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment<'a> {
    pub store: HashMap<String, Object<'a>>,
    pub outer: Option<Rc<RefCell<Environment<'a>>>>,
}
impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }
    pub fn new_with_outer(outer: Rc<RefCell<Environment<'a>>>) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn set(&mut self, key: String, value: Object<'a>) {
        self.store
            .entry(key)
            .and_modify(|x| *x = value.clone())
            .or_insert(value.clone());
    }

    // Recursively searches for the key in the "parent" environment
    pub fn get(&self, key: &str) -> Result<Object<'a>, EvalErr> {
        let res = self.store.get(key);
        if res.is_none() {
            if let Some(outer) = &self.outer {
                let val = outer.borrow().get(key)?.clone();
                return Ok(val);
            }
            return Err(EvalErr::IdentifierNotFound(key.to_string()));
        }
        return Ok(res.unwrap().clone());
    }
}
