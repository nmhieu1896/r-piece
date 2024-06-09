use std::{ops::Deref, os::macos::raw::stat};

use crate::{
    ast::ast::{Integer, Node, NodeType, Program, Statement},
    errors::eval_errs::EvalErr,
};

use super::object::Object;

pub fn eval(node: &Box<dyn Node>) -> Result<Object, EvalErr> {
    println!("{:?}", node);
    match node.node_type() {
        NodeType::Program => {
            return eval_statement(&node.as_any().downcast_ref::<Program>().unwrap().statements)
        }
        NodeType::String => {
            return Ok(Object::String(
                node.as_any().downcast_ref::<String>().unwrap().clone(),
            ))
        }
        NodeType::Int => {
            return Ok(Object::Number(
                node.as_any().downcast_ref::<Integer>().unwrap().clone(),
            ))
        }
        NodeType::Bool => {
            return Ok(Object::Boolean(
                node.as_any().downcast_ref::<bool>().unwrap().clone(),
            ))
        }
        _ => return Err(EvalErr::NotImplemented),
    }
}

fn eval_statement(statements: &Vec<Box<dyn Statement>>) -> Result<Object, EvalErr> {
    let mut result = Object::Null;
    for &stmt in statements.iter() {
        let node: Box<dyn Node> = stmt;

        // result = eval(stmt)?;
    }

    return Ok(result);
}
