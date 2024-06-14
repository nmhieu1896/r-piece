use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::ast::{IfExpression, Node, NodeTrait, NodeType, Statement},
    errors::eval_errs::EvalErr,
    lexer::token::TOKEN,
};

use super::{environment::Environment, object::Object};

pub fn eval<'a>(node: Node, env: Rc<RefCell<Environment<'a>>>) -> Result<Object<'a>, EvalErr> {
    match node.node_type() {
        NodeType::Program => {
            return eval_statements(
                &node.to_statement()?.to_program()?.statements,
                Rc::clone(&env),
            )
        }
        NodeType::ExpressionStatement => {
            let expr = node.to_statement()?.to_exp_stmt()?.expression;
            if expr.is_none() {
                return Ok(Object::Null);
            }
            return eval(Node::Expression(expr.unwrap()), Rc::clone(&env));
        }
        NodeType::PrefixExpression => {
            let expr = node.to_expression()?.to_prefix()?;
            return eval_prefix_expression(
                expr.token.clone(),
                eval(Node::Expression(expr.right), Rc::clone(&env))?,
            );
        }
        NodeType::InfixExpression => {
            let expr = node.to_expression()?.to_infix()?;
            let left = eval(Node::Expression(expr.left), Rc::clone(&env))?;
            let right = eval(Node::Expression(expr.right), Rc::clone(&env))?;
            return eval_infix_expression(expr.operator.clone(), left, right);
        }
        NodeType::ReturnStatement => {
            let expr = node.to_statement()?.to_return()?;
            if expr.expression.is_none() {
                return Ok(Object::Return(Box::new(Object::Null)));
            }
            return Ok(Object::Return(Box::new(eval(
                Node::Expression(expr.expression.unwrap()),
                Rc::clone(&env),
            )?)));
        }
        NodeType::IfExpression => {
            let expr = node.to_expression()?.to_if()?;
            return eval_if_expression(expr, Rc::clone(&env));
        }
        NodeType::BlockStatement => {
            let expr = node.to_statement()?.to_block()?;
            return eval_statements(&expr.statements, Rc::clone(&env));
        }
        NodeType::LetStatement => {
            let expr = node.to_statement()?.to_let()?;
            let value = eval(Node::Expression(expr.value), Rc::clone(&env))?;
            env.borrow_mut().set(expr.name.clone(), value);
            return Ok(Object::Null);
        }
        NodeType::Identifier => {
            let key = node.to_expression()?.to_ident()?;
            let borrow_env = env.borrow();
            let value = borrow_env.get(&key);
            if value.is_none() {
                return Err(EvalErr::IdentifierNotFound(key));
            }

            return Ok(value.unwrap().clone());
        }
        NodeType::Number => return Ok(Object::Number(node.to_expression()?.to_num()?)),
        NodeType::Bool => return Ok(Object::Boolean(node.to_expression()?.to_bool()?)),
        _ => {
            return Err(EvalErr::NotImplemented(format!(
                "{:?} is not implemented",
                node.node_type(),
            )))
        }
    }
}

fn eval_statements<'a>(
    statements: &Vec<Statement>,
    env: Rc<RefCell<Environment<'a>>>,
) -> Result<Object<'a>, EvalErr> {
    let mut result = Object::Null;

    for stmt in statements.iter() {
        result = eval(Node::Statement(stmt.clone()), Rc::clone(&env))?;
        if result.is_return() {
            return Ok(result);
        }
    }

    return Ok(result);
}

fn eval_prefix_expression(operator: TOKEN, right: Object) -> Result<Object, EvalErr> {
    match operator {
        TOKEN::BANG => Ok(eval_bang_expression(right)),
        TOKEN::MINUS => match right {
            Object::Number(n) => Ok(Object::Number(-n)),
            _ => Err(EvalErr::MinusPrefix(format!("{:?}", right))),
        },
        _ => Err(EvalErr::NotImplemented(format!(
            "{:?} is not implemented for prefix expression",
            operator
        ))),
    }
}
fn is_truthy(value: Object) -> bool {
    match value {
        Object::Boolean(b) => b,
        Object::Null => false,
        Object::Number(n) => n != 0,
        _ => true,
    }
}

fn eval_bang_expression(value: Object) -> Object {
    Object::Boolean(!is_truthy(value))
}
fn eval_eq_expression<'a>(
    operator: TOKEN,
    left: Object<'a>,
    right: Object<'a>,
) -> Result<Object<'a>, EvalErr> {
    match left {
        Object::Number(n) if operator == TOKEN::EQ => Ok(Object::Boolean(n == right.as_int()?)),
        Object::Number(n) if operator == TOKEN::NotEQ => Ok(Object::Boolean(n != right.as_int()?)),
        Object::Boolean(b) if operator == TOKEN::EQ => Ok(Object::Boolean(b == right.as_bool()?)),
        Object::Boolean(b) if operator == TOKEN::NotEQ => {
            Ok(Object::Boolean(b != right.as_bool()?))
        }
        _ => Err(EvalErr::NotImplemented(format!(
            "{:?} cant be compared",
            left
        ))),
    }
}

fn eval_infix_expression<'a>(
    operator: TOKEN,
    left: Object<'a>,
    right: Object<'a>,
) -> Result<Object<'a>, EvalErr> {
    match operator {
        TOKEN::PLUS => Ok(Object::Number(
            left.as_int_with(TOKEN::PLUS)? + right.as_int_with(TOKEN::PLUS)?,
        )),
        TOKEN::MINUS => Ok(Object::Number(
            left.as_int_with(TOKEN::MINUS)? - right.as_int_with(TOKEN::MINUS)?,
        )),
        TOKEN::ASTERISK => Ok(Object::Number(
            left.as_int_with(TOKEN::ASTERISK)? * right.as_int_with(TOKEN::ASTERISK)?,
        )),
        TOKEN::SLASH if right.as_int_with(TOKEN::SLASH)? == 0 => Err(EvalErr::DivideByZero),
        TOKEN::SLASH => Ok(Object::Number(
            left.as_int_with(TOKEN::SLASH)? / right.as_int_with(TOKEN::SLASH)?,
        )),
        TOKEN::GT => Ok(Object::Boolean(
            left.as_int_with(TOKEN::GT)? > right.as_int_with(TOKEN::GT)?,
        )),
        TOKEN::LT => Ok(Object::Boolean(
            left.as_int_with(TOKEN::LT)? < right.as_int_with(TOKEN::LT)?,
        )),
        TOKEN::EQ => eval_eq_expression(operator, left, right),
        TOKEN::NotEQ => eval_eq_expression(operator, left, right),
        _ => Err(EvalErr::NotImplemented(format!(
            "{:?} is not implemented for infix expression",
            operator
        ))),
    }
}

fn eval_if_expression<'a>(
    expression: IfExpression,
    env: Rc<RefCell<Environment<'a>>>,
) -> Result<Object<'a>, EvalErr> {
    let condition = eval(Node::Expression(expression.condition), Rc::clone(&env))?;
    if is_truthy(condition) {
        return eval_statements(&expression.consequence.statements, Rc::clone(&env));
    }
    if let Some(alternative) = &expression.alternative {
        return eval_statements(&alternative.statements, Rc::clone(&env));
    }
    return Ok(Object::Null);
}
