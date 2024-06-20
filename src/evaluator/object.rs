use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::ast::{BlockStatement, Identifier},
    // errors::coerce_errs::CoerceErr,
};

use super::environment::Environment;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Object<'a> {
    Number(i64),
    Identifier(Identifier),
    String(String),
    Builtin(String), // get function from builtin
    Boolean(bool),
    Array(Rc<RefCell<Vec<Object<'a>>>>),
    Null,
    Return(Box<Object<'a>>),
    Function(Function<'a>),
}

impl<'a> Object<'a> {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    pub fn is_return(&self) -> bool {
        match self {
            Object::Return(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub params: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Rc<RefCell<Environment<'a>>>,
}
impl<'a> Function<'a> {
    pub fn new(
        params: Vec<Identifier>,
        body: BlockStatement,
        env: Rc<RefCell<Environment<'a>>>,
    ) -> Self {
        Self { params, body, env }
    }
}

impl<'a> PartialEq for Function<'a> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
