use crate::{
    ast::ast::{upcast_trait, ExpressionStatement, Integer, Node, NodeType, Program, Statement},
    errors::eval_errs::EvalErr,
};

use super::object::Object;

pub fn eval(node: &dyn Node) -> Result<Object, EvalErr> {
    match node.node_type() {
        NodeType::Program => {
            return eval_statement(&node.as_any().downcast_ref::<Program>().unwrap().statements)
        }
        NodeType::ExpressionStatement => {
            return eval(
                node.as_any()
                    .downcast_ref::<ExpressionStatement>()
                    .unwrap()
                    .expression
                    .as_deref()
                    .unwrap(),
            )
        }
        NodeType::Identifier => {
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
    let vec = statements
        .iter()
        .map(|x| upcast_trait(x.as_ref()))
        .collect::<Vec<&dyn Node>>();
    for &stmt in vec.iter() {
        result = eval(stmt)?;
    }

    return Ok(result);
}
