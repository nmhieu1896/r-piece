use crate::{
    ast::ast::{BlockStatement, Identifier},
    errors::coerce_errs::CoerceErr,
    lexer::token::TOKEN,
};

use super::environment::Environment;

// #[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Object<'a> {
    Number(i64),
    Identifier(String),
    Boolean(bool),
    Null,
    Return(Box<Object<'a>>),
    Function(Function<'a>),
}

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub params: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: &'a Environment<'a>,
}
impl<'a> Function<'a> {
    pub fn new(params: Vec<Identifier>, body: BlockStatement, env: &'a Environment<'a>) -> Self {
        Self { params, body, env }
    }
}
// impl PartialEq for Function<'a> {
impl<'a> PartialEq for Function<'a> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<'a> Object<'a> {
    pub fn as_int(&self) -> Result<i64, CoerceErr> {
        match self {
            Object::Number(n) => Ok(*n as i64),
            _ => Err(CoerceErr::ToInt(format!("{:?}", self))),
        }
    }
    pub fn as_int_with(&self, operator: TOKEN) -> Result<i64, CoerceErr> {
        match self {
            Object::Number(n) => Ok(*n as i64),
            _ => Err(CoerceErr::Operator(format!("{:?}", self), operator)),
        }
    }
    pub fn as_bool(&self) -> Result<bool, CoerceErr> {
        match self {
            Object::Boolean(b) => Ok(*b),
            _ => Err(CoerceErr::ToBool(format!("{:?}", self))),
        }
    }
    pub fn is_return(&self) -> bool {
        match self {
            Object::Return(_) => true,
            _ => false,
        }
    }
}
