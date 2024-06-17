use crate::{
    ast::ast::{CallExpression, Expression, InfixExpression},
    errors::parser_errs::ParseErr,
    lexer::token::TOKEN,
};

use super::{
    parse_expression::parse_expression,
    parser::{Parser, Precedence},
};

pub fn parse_infix_expression<'a>(
    parser: &mut Parser<'a>,
    left: Expression,
) -> Result<Expression, ParseErr> {
    let operator = parser.cur_token.clone();
    let precedence = parser.cur_precedence();
    parser.next_token();
    let right = parse_expression(parser, precedence)?;
    let inf_exp = InfixExpression::new(left, operator, right);
    Ok(Expression::Infix(Box::new(inf_exp)))
}

pub fn parse_call_expression<'a>(
    parser: &mut Parser<'a>,
    function: Expression,
) -> Result<Expression, ParseErr> {
    let mut call = CallExpression::new(function);
    call.arguments = parse_call_args(parser)?;
    if parser.peek_token.is_same_with(TOKEN::RPAREN) {
        parser.next_token();
    }
    return Ok(Expression::Call(Box::new(call)));
}

pub fn parse_call_args<'a>(parser: &mut Parser<'a>) -> Result<Vec<Expression>, ParseErr> {
    let mut args = vec![];

    while !parser.peek_token.is_same_with(TOKEN::RPAREN) {
        if parser.peek_token.is_same_with(TOKEN::EOF) {
            return Err(ParseErr::CALL("RPAREN".into(), parser.peek_token.clone()));
        }
        parser.next_token();
        let expression = parse_expression(parser, Precedence::LOWEST);
        if let Some(exp) = parser.result_to_option(expression)? {
            args.push(exp);
        }
    }

    return Ok(args);
}
