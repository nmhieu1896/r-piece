use crate::{
    ast::ast::{
        ExpressionStatement, Identifier, LetStatement, ReassignStatement, ReturnStatement,
        Statement,
    },
    errors::parser_errs::ParseErr,
    lexer::token::TOKEN,
};

use super::{
    parse_expression::parse_expression,
    parser::{Parser, Precedence},
};

pub fn parse_statement<'a>(parser: &mut Parser<'a>) -> Result<Statement, ParseErr> {
    match parser.cur_token {
        TOKEN::LET => parse_let_statement(parser),
        TOKEN::RETURN => parse_return_statement(parser),
        TOKEN::IDENT(_) if parser.peek_token == TOKEN::ASSIGN => parse_reassign_statement(parser),
        _ => Ok(parse_expression_statement(parser)?),
    }
}

pub fn parse_let_statement<'a>(parser: &mut Parser<'a>) -> Result<Statement, ParseErr> {
    if !parser
        .peek_token
        .is_same_with(TOKEN::IDENT(Identifier(String::new())))
    {
        return Err(ParseErr::LET("IDENT".into(), parser.peek_token.clone()));
    }
    parser.next_token(); // to ident token
    let name = parser.cur_token.literal();

    parser.next_token(); //to assign token
    if !parser.cur_token.is_same_with(TOKEN::ASSIGN) {
        return Err(ParseErr::LET("ASSIGN".into(), parser.cur_token.clone()));
    }

    parser.next_token(); //to expression
    let value = parse_expression(parser, Precedence::LOWEST)?;
    if parser.peek_token.is_same_with(TOKEN::SEMICOLON) {
        parser.next_token();
    }

    let stmt = LetStatement::new(Identifier(name), value);

    return Ok(Statement::Let(stmt));
}

pub fn parse_reassign_statement<'a>(parser: &mut Parser<'a>) -> Result<Statement, ParseErr> {
    let name = parser.cur_token.literal();
    //Dont need to check error because match case in parse_statement've already done it
    parser.next_token(); //to assign token
    parser.next_token(); //to expression
    let value = parse_expression(parser, Precedence::LOWEST)?;
    let stmt = ReassignStatement::new(Identifier(name), value);
    return Ok(Statement::Reassign(stmt));
}

pub fn parse_return_statement<'a>(parser: &mut Parser<'a>) -> Result<Statement, ParseErr> {
    let mut stmt = ReturnStatement::new();

    parser.next_token();
    let expression = parse_expression(parser, Precedence::LOWEST);
    stmt.expression = parser.result_to_option(expression)?;
    if parser.peek_token.is_same_with(TOKEN::SEMICOLON) {
        parser.next_token();
    }
    return Ok(Statement::Return(stmt));
}

pub fn parse_expression_statement<'a>(parser: &mut Parser<'a>) -> Result<Statement, ParseErr> {
    let mut stmt = ExpressionStatement::new(parser.cur_token.clone());
    let expression = parse_expression(parser, Precedence::LOWEST);
    stmt.expression = parser.result_to_option(expression)?;

    if parser.peek_token.is_same_with(TOKEN::SEMICOLON) {
        parser.next_token();
    }

    return Ok(Statement::Expression(stmt));
}

pub fn parse_block_statement<'a>(parser: &mut Parser<'a>) -> Result<Vec<Statement>, ParseErr> {
    if parser.cur_token.is_same_with(TOKEN::RBRACE) {
        return Err(ParseErr::BLOCK("RBRACE".into(), parser.cur_token.clone()));
    }
    let mut block_stmts = Vec::new();

    parser.next_token(); // to move on from "{"
    while !parser.cur_token.is_same_with(TOKEN::RBRACE) {
        if parser.cur_token.is_same_with(TOKEN::EOF) {
            return Err(ParseErr::BLOCK("LBRACE".into(), parser.cur_token.clone()));
        }

        let stmt = parse_statement(parser)?;
        block_stmts.push(stmt);
        parser.next_token();
    }

    return Ok(block_stmts);
}
