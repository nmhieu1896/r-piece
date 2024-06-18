use crate::{errors::eval_errs::EvalErr, lexer::token::TOKEN};

use super::object::Object;

pub fn eval_infix_expression<'a>(
    operator: TOKEN,
    left: Object<'a>,
    right: Object<'a>,
) -> Result<Object<'a>, EvalErr> {
    match operator {
        TOKEN::PLUS => eval_plus_expression(left, right),
        TOKEN::MINUS => eval_substract_expression(left, right),
        TOKEN::ASTERISK => eval_multiply_expression(left, right),
        TOKEN::SLASH => eval_div_expression(left, right),

        TOKEN::LT | TOKEN::GT => eval_order_expression(operator, left, right),
        TOKEN::EQ | TOKEN::NotEQ => eval_eq_expression(operator, left, right),

        _ => Err(EvalErr::NotImplemented(format!(
            "{:?} is not implemented for infix expression",
            operator
        ))),
    }
}

fn eval_plus_expression<'a>(left: Object<'a>, right: Object<'a>) -> Result<Object<'a>, EvalErr> {
    match (left, right) {
        (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 + n2)),
        (Object::String(s1), Object::String(s2)) => {
            let str = s1.clone() + &s2;
            Ok(Object::String(str))
        }
        (l, r) => Err(EvalErr::PlusError(l.to_string(), r.to_string())),
    }
}
fn eval_substract_expression<'a>(
    left: Object<'a>,
    right: Object<'a>,
) -> Result<Object<'a>, EvalErr> {
    match (left, right) {
        (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 - n2)),
        (l, r) => Err(EvalErr::SubstractError(l.to_string(), r.to_string())),
    }
}

fn eval_multiply_expression<'a>(
    left: Object<'a>,
    right: Object<'a>,
) -> Result<Object<'a>, EvalErr> {
    match (left, right) {
        (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 * n2)),
        (Object::Number(n), Object::String(s)) | (Object::String(s), Object::Number(n)) => {
            let mut str = String::new();
            for _ in 0..n {
                str.push_str(&s);
            }
            Ok(Object::String(str))
        }
        (l, r) => Err(EvalErr::MultiplyError(l.to_string(), r.to_string())),
    }
}

fn eval_div_expression<'a>(left: Object<'a>, right: Object<'a>) -> Result<Object<'a>, EvalErr> {
    match (left, right) {
        (_, Object::Number(n2)) if n2 == 0 => Err(EvalErr::DivideByZero),
        (Object::Number(n1), Object::Number(n2)) => Ok(Object::Number(n1 / n2)),
        (l, r) => Err(EvalErr::DivideError(l.to_string(), r.to_string())),
    }
}

fn eval_order_expression<'a>(
    operator: TOKEN,
    left: Object<'a>,
    right: Object<'a>,
) -> Result<Object<'a>, EvalErr> {
    match (left, right) {
        (Object::Number(n1), Object::Number(n2)) => Ok(Object::Boolean(
            (n1 > n2 && operator == TOKEN::GT) || (n1 < n2 && operator == TOKEN::LT),
        )),
        (Object::String(s1), Object::String(s2)) => Ok(Object::Boolean(
            (s1 > s2 && operator == TOKEN::GT) || (s1 < s2 && operator == TOKEN::LT),
        )),
        (l, r) => Err(EvalErr::Order(l.to_string(), r.to_string())),
    }
}

fn eval_eq_expression<'a>(
    operator: TOKEN,
    left: Object<'a>,
    right: Object<'a>,
) -> Result<Object<'a>, EvalErr> {
    match (left, right) {
        (Object::Number(n1), Object::Number(n2)) => Ok(Object::Boolean(
            (n1 == n2 && operator == TOKEN::EQ) || (n1 != n2 && operator == TOKEN::NotEQ),
        )),
        (Object::String(s1), Object::String(s2)) => Ok(Object::Boolean(
            (s1 == s2 && operator == TOKEN::EQ) || (s1 != s2 && operator == TOKEN::NotEQ),
        )),
        (Object::Boolean(b1), Object::Boolean(b2)) => Ok(Object::Boolean(
            (b1 == b2 && operator == TOKEN::EQ) || (b1 != b2 && operator == TOKEN::NotEQ),
        )),
        (l, r) => Err(EvalErr::Equal(l.to_string(), r.to_string())),
    }
}
