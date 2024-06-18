use crate::{
    ast::ast::{Expression, Identifier, Program},
    errors::parser_errs::ParseErr,
    lexer::{lexer::Lexer, token::TOKEN},
};

use std::collections::HashMap;

use super::{
    parse_infix::{parse_call_expression, parse_infix_expression},
    parse_prefix::{
        parse_boolean_literal, parse_function_literal, parse_group_expression, parse_identifier,
        parse_if_expression, parse_int_literal, parse_prefix_expression, parse_string,
    },
    parse_statement::parse_statement,
};

type PrefixParseFn = fn(&mut Parser) -> Result<Expression, ParseErr>;
type InfixParseFn = fn(&mut Parser, Expression) -> Result<Expression, ParseErr>;
pub enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}
impl Precedence {
    pub fn from_token(token: TOKEN) -> Self {
        match token {
            TOKEN::EQ | TOKEN::NotEQ => Self::EQUALS,
            TOKEN::LT | TOKEN::GT => Self::LESSGREATER,
            TOKEN::PLUS | TOKEN::MINUS => Self::SUM,
            TOKEN::SLASH | TOKEN::ASTERISK => Self::PRODUCT,
            TOKEN::LPAREN => Self::CALL,
            _ => Self::LOWEST,
        }
    }
    pub fn order(&self) -> i32 {
        match self {
            Precedence::LOWEST => 1,
            Precedence::EQUALS => 2,
            Precedence::LESSGREATER => 3,
            Precedence::SUM => 4,
            Precedence::PRODUCT => 5,
            Precedence::PREFIX => 6,
            Precedence::CALL => 7,
        }
    }
}

#[derive(Debug)]
pub struct Parser<'a> {
    l: Lexer<'a>,
    pub cur_token: TOKEN,
    pub peek_token: TOKEN,
    pub prefix_parse_fns: HashMap<String, PrefixParseFn>,
    pub infix_parse_fns: HashMap<String, InfixParseFn>,
}

impl<'a> Parser<'a> {
    pub fn new(l: Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            l,
            cur_token: TOKEN::EOF,
            peek_token: TOKEN::EOF,
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        // PREFIX PARSERS
        p.register_prefix(TOKEN::IDENT(Identifier("".into())), parse_identifier);
        p.register_prefix(TOKEN::STRING("".into()), parse_string);
        p.register_prefix(TOKEN::NUMBER(0), parse_int_literal);
        p.register_prefix(TOKEN::TRUE, parse_boolean_literal);
        p.register_prefix(TOKEN::FALSE, parse_boolean_literal);
        p.register_prefix(TOKEN::BANG, parse_prefix_expression);
        p.register_prefix(TOKEN::MINUS, parse_prefix_expression);
        p.register_prefix(TOKEN::LPAREN, parse_group_expression);
        p.register_prefix(TOKEN::IF, parse_if_expression);
        p.register_prefix(TOKEN::FUNCTION, parse_function_literal);
        // INFIX PARSERS
        p.register_infix(TOKEN::PLUS, parse_infix_expression);
        p.register_infix(TOKEN::MINUS, parse_infix_expression);
        p.register_infix(TOKEN::SLASH, parse_infix_expression);
        p.register_infix(TOKEN::ASTERISK, parse_infix_expression);
        p.register_infix(TOKEN::EQ, parse_infix_expression);
        p.register_infix(TOKEN::NotEQ, parse_infix_expression);
        p.register_infix(TOKEN::LT, parse_infix_expression);
        p.register_infix(TOKEN::GT, parse_infix_expression);
        p.register_infix(TOKEN::LPAREN, parse_call_expression);
        //Read two token so current token and peek token are both set
        p.next_token();
        p.next_token();
        return p;
    }
    fn register_prefix(&mut self, token: TOKEN, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token.to_type_name(), func);
    }

    fn register_infix(&mut self, token: TOKEN, func: InfixParseFn) {
        self.infix_parse_fns.insert(token.to_type_name(), func);
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn peek_precedence(&self) -> Precedence {
        Precedence::from_token(self.peek_token.clone())
    }
    pub fn cur_precedence(&self) -> Precedence {
        Precedence::from_token(self.cur_token.clone())
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseErr> {
        let mut program = Program { statements: vec![] };

        while self.cur_token != TOKEN::EOF {
            let stmt = parse_statement(self)?;
            program.statements.push(stmt);
            self.next_token();
        }

        return Ok(program);
    }

    pub fn result_to_option(
        &self,
        result: Result<Expression, ParseErr>,
    ) -> Result<Option<Expression>, ParseErr> {
        match result {
            Ok(ok_exp) => Ok(Some(ok_exp)),
            Err(ParseErr::None) => Ok(None),
            Err(err) => {
                return Err(err);
            }
        }
    }
}
