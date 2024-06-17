use crate::{
    ast::ast::{BlockStatement, Expression, FunctionLiteral, IfExpression, PrefixExpression},
    errors::parser_errs::ParseErr,
    lexer::token::TOKEN,
};

use super::{
    parse_expression::parse_expression,
    parse_statement::parse_block_statement,
    parser::{Parser, Precedence},
};

pub fn parse_identifier<'a>(parser: &mut Parser<'a>) -> Result<Expression, ParseErr> {
    Ok(Expression::Identifier(parser.cur_token.literal()))
}

pub fn parse_int_literal<'a>(parser: &mut Parser<'a>) -> Result<Expression, ParseErr> {
    Ok(Expression::Number(
        parser.cur_token.literal().parse::<i64>().unwrap(),
    ))
}

pub fn parse_boolean_literal<'a>(parser: &mut Parser<'a>) -> Result<Expression, ParseErr> {
    Ok(Expression::Bool(
        parser.cur_token.literal().parse::<bool>().unwrap(),
    ))
}

pub fn parse_prefix_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression, ParseErr> {
    let token = parser.cur_token.clone();
    parser.next_token();
    let right_exp = parse_expression(parser, Precedence::PREFIX)?;
    let expression = PrefixExpression::new(token, right_exp);
    Ok(Expression::Prefix(Box::new(expression)))
}

pub fn parse_group_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression, ParseErr> {
    parser.next_token(); // to move on from "("
    let expression = parse_expression(parser, Precedence::LOWEST);

    if !parser.peek_token.is_same_with(TOKEN::RPAREN) {
        return Err(ParseErr::GROUP("RPAREN".into(), parser.peek_token.clone()));
    }
    parser.next_token(); // to move on from ")"
    return expression;
}

pub fn parse_if_expression<'a>(parser: &mut Parser<'a>) -> Result<Expression, ParseErr> {
    parser.next_token();
    let condition = parse_expression(parser, Precedence::LOWEST)?;
    let mut expression = IfExpression::new(condition);

    if !parser.peek_token.is_same_with(TOKEN::LBRACE) {
        return Err(ParseErr::IF("LBRACE".into(), parser.peek_token.clone()));
    };
    parser.next_token();
    expression.consequence = BlockStatement::new(parse_block_statement(parser)?);

    if parser.peek_token.is_same_with(TOKEN::ELSE) {
        parser.next_token(); // move to ELSE

        expression.alternative = if parser.peek_token.is_same_with(TOKEN::LBRACE) {
            parser.next_token(); // move on from ELSE
            Some(BlockStatement::new(parse_block_statement(parser)?))
        } else if parser.peek_token.is_same_with(TOKEN::IF) {
            None
        } else {
            let expect = "IF or LBRACE";
            return Err(ParseErr::ELSE(expect.into(), parser.peek_token.clone()));
        }
    }

    return Ok(Expression::If(Box::new(expression)));
}

// call this when current token is "("
pub fn parse_function_literal<'a>(parser: &mut Parser<'a>) -> Result<Expression, ParseErr> {
    parser.next_token();
    let mut function = FunctionLiteral::new(parse_fn_parameters(parser)?);
    function.body = BlockStatement::new(parse_block_statement(parser)?);
    return Ok(Expression::Function(Box::new(function)));
}

// when calling this, current token must be "(" or LPAREN
pub fn parse_fn_parameters<'a>(parser: &mut Parser<'a>) -> Result<Vec<String>, ParseErr> {
    let mut identifiers = Vec::new();
    parser.next_token(); // move on from '(',
    while !parser.cur_token.is_same_with(TOKEN::RPAREN) {
        match parser.cur_token {
            TOKEN::IDENT(ref name) => {
                identifiers.push(name.clone());
                parser.next_token()
            }
            _ => {
                return Err(ParseErr::FN("IDENT".into(), parser.cur_token.clone()));
            }
        }
        if parser.cur_token.is_same_with(TOKEN::COMMA) {
            parser.next_token()
        }
    }
    parser.next_token(); // move on from (

    return Ok(identifiers);
}
