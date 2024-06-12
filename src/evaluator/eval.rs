use crate::{
    ast::ast::{
        upcast_trait, BlockStatement, Boolean, ExpressionStatement, Identifier, IfExpression,
        InfixExpression, Integer, Node, NodeType, PrefixExpression, Program, ReturnStatement,
        Statement,
    },
    errors::eval_errs::EvalErr,
    lexer::token::TOKEN,
};

use super::object::Object;

pub fn eval(node: &dyn Node) -> Result<Object, EvalErr> {
    match node.node_type() {
        NodeType::Program => {
            return eval_statements(&node.as_any().downcast_ref::<Program>().unwrap().statements)
        }
        NodeType::ExpressionStatement => {
            let expr = node
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap()
                .expression
                .as_deref();
            if expr.is_none() {
                return Ok(Object::Null);
            }
            return eval(expr.unwrap());
        }
        NodeType::PrefixExpression => {
            let expr = node.as_any().downcast_ref::<PrefixExpression>().unwrap();
            return eval_prefix_expression(expr.token.clone(), eval(expr.right.as_ref())?);
        }
        NodeType::InfixExpression => {
            let expr = node.as_any().downcast_ref::<InfixExpression>().unwrap();
            return eval_infix_expression(
                expr.operator.clone(),
                eval(expr.left.as_ref())?,
                eval(expr.right.as_ref())?,
            );
        }
        NodeType::ReturnStatement => {
            let expr = node.as_any().downcast_ref::<ReturnStatement>().unwrap();
            if expr.expression.is_none() {
                return Ok(Object::Null);
            }
            return Ok(Object::Return(Box::new(eval(
                expr.expression.as_deref().unwrap(),
            )?)));
        }
        NodeType::IfExpression => {
            let expr = node.as_any().downcast_ref::<IfExpression>().unwrap();
            return eval_if_expression(expr);
        }
        NodeType::BlockStatement => {
            let expr = node.as_any().downcast_ref::<BlockStatement>().unwrap();
            return eval_statements(&expr.statements);
        }
        NodeType::Identifier => {
            return Ok(Object::Identifier(
                node.as_any().downcast_ref::<Identifier>().unwrap().clone(),
            ))
        }
        NodeType::Int => {
            return Ok(Object::Number(
                node.as_any().downcast_ref::<Integer>().unwrap().clone(),
            ))
        }
        NodeType::Bool => {
            return Ok(Object::Boolean(
                node.as_any().downcast_ref::<Boolean>().unwrap().clone(),
            ))
        }
        _ => {
            return Err(EvalErr::NotImplemented(format!(
                "{:?} is not implemented",
                node.node_type(),
            )))
        }
    }
}

fn eval_statements(statements: &Vec<Box<dyn Statement>>) -> Result<Object, EvalErr> {
    let mut result = Object::Null;
    let vec = statements
        .iter()
        .map(|x| upcast_trait(x.as_ref()))
        .collect::<Vec<&dyn Node>>();
    for &stmt in vec.iter() {
        result = eval(stmt)?;
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
            _ => Err(EvalErr::MinusPrefix(right)),
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
fn eval_eq_expression(operator: TOKEN, left: Object, right: Object) -> Result<Object, EvalErr> {
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

fn eval_infix_expression(operator: TOKEN, left: Object, right: Object) -> Result<Object, EvalErr> {
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

fn eval_if_expression(expression: &IfExpression) -> Result<Object, EvalErr> {
    let condition = eval(expression.condition.as_ref())?;
    if is_truthy(condition) {
        return eval_statements(&expression.consequence.statements);
    }
    if let Some(alternative) = &expression.alternative {
        return eval_statements(&alternative.statements);
    }
    return Ok(Object::Null);
}
