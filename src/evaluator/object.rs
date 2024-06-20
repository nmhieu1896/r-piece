use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::ast::{BlockStatement, Identifier, NodeTrait},
    errors::eval_errs::EvalErr,
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
    pub fn to_num(&self) -> Result<i64, EvalErr> {
        match self {
            Object::Number(n) => Ok(*n),
            anything => Err(EvalErr::CoerceObject(
                anything.to_string(),
                "number".to_string(),
            )),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Object::String(s) => s.clone(),
            Object::Number(n) => n.to_string(),
            Object::Identifier(i) => i.0.clone(),
            Object::Boolean(b) => b.to_string(),
            Object::Array(a) => {
                let mut str = String::from("[");
                str.push_str(
                    &a.as_ref()
                        .borrow()
                        .clone()
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                str.push(']');
                return str;
            }
            Object::Null => "Null".to_string(),
            Object::Return(r) => r.to_string(),
            Object::Function(f) => {
                let mut str = String::from("fn (");
                str.push_str(
                    &f.params
                        .iter()
                        .map(|x| x.0.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                str.push(')');
                str.push_str(&f.body.clone().to_str());
                return str;
            }
            Object::Builtin(s) => format!("builtins({})", s),
        }
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
