use crate::{errors::coerce_errs::CoerceErr, lexer::token::TOKEN};

// #[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(i64),
    Identifier(String),
    // String(String),
    Boolean(bool),
    Null,
    Return(Box<Object>),
    // Function(fn(Vec<Object>) -> Object),
}

impl Object {
    pub fn as_int(&self) -> Result<i64, CoerceErr> {
        match self {
            Object::Number(n) => Ok(*n as i64),
            _ => Err(CoerceErr::ToInt(self.clone().into())),
        }
    }
    pub fn as_int_with(&self, operator: TOKEN) -> Result<i64, CoerceErr> {
        match self {
            Object::Number(n) => Ok(*n as i64),
            _ => Err(CoerceErr::Operator(self.clone().into(), operator)),
        }
    }
    pub fn as_bool(&self) -> Result<bool, CoerceErr> {
        match self {
            Object::Boolean(b) => Ok(*b),
            _ => Err(CoerceErr::ToBool(self.clone().into())),
        }
    }
    pub fn is_return(&self) -> bool {
        match self {
            Object::Return(_) => true,
            _ => false,
        }
    }
}
