use std::collections::HashMap;

use crate::{
    ast::ast::{Expression, ExpressionStatement, LetStatement, Node, Program, Statement},
    lexer::{lexer::Lexer, token::TOKEN},
};

impl Expression for String {
    fn expression_node(&self) {}
}
impl Node for String {
    fn token_literal(&self) -> String {
        self.clone()
    }
}

type prefix_parse_fn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type infix_parse_fn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;
pub enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

#[derive(Debug)]
pub struct Parser {
    l: Lexer,
    errors: Vec<String>,
    cur_token: TOKEN,
    peek_token: TOKEN,
    prefix_parse_fns: HashMap<String, prefix_parse_fn>,
    infix_parse_fns: HashMap<String, infix_parse_fn>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            errors: vec![],
            cur_token: TOKEN::EOF,
            peek_token: TOKEN::EOF,
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        //
        p.prefix_parse_fns.insert(
            TOKEN::IDENT(String::new()).to_type_name(),
            Parser::parse_identifier,
        );
        //Read two token so current token and peek token are both set
        p.next_token();
        p.next_token();
        return p;
    }
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token != TOKEN::EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }

        return program;
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token {
            TOKEN::LET => {
                if let Some(stmt) = self.parse_let_statement() {
                    Some(Box::new(stmt))
                } else {
                    None
                }
            }
            TOKEN::RETURN => {
                if let Some(stmt) = self.parse_return_statement() {
                    Some(Box::new(stmt))
                } else {
                    None
                }
            }
            _ => {
                if let Some(stmt) = self.parse_expression_statement() {
                    Some(Box::new(stmt))
                } else {
                    None
                }
            }
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let mut stmt = LetStatement::new(TOKEN::LET);
        // get IDENT
        if !self.peek_token.is_same_with(TOKEN::IDENT(String::new())) {
            self.errors.push(format!(
                "Expected next token to be IDENT, got {:?}",
                self.peek_token
            ));
            return None;
        }
        stmt.name = Some(self.peek_token.literal());

        // get ASSIGN
        self.next_token();
        if !self.peek_token.is_same_with(TOKEN::ASSIGN) {
            self.errors.push(format!(
                "Expected next token to be ASSIGN, got {:?}",
                self.peek_token
            ));
            return None;
        }

        while !self.cur_token.is_same_with(TOKEN::SEMICOLON) {
            self.next_token();
        }

        return Some(stmt);
    }

    pub fn parse_return_statement(&mut self) -> Option<LetStatement> {
        let stmt = LetStatement::new(TOKEN::RETURN);

        self.next_token();
        while !self.cur_token.is_same_with(TOKEN::SEMICOLON) {
            self.next_token();
        }

        return Some(stmt);
    }

    pub fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let mut stmt = ExpressionStatement::new(self.cur_token.clone());
        stmt.expression = self.parse_expression(Precedence::LOWEST);

        if self.peek_token.is_same_with(TOKEN::SEMICOLON) {
            self.next_token();
        }

        return Some(stmt);
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let prefix = self.prefix_parse_fns.get(&self.cur_token.to_type_name());

        if prefix.is_none() {
            return None;
        }

        let left_exp = prefix.unwrap()(self);
        return left_exp;
    }

    fn register_prefix(&mut self, token: TOKEN, func: prefix_parse_fn) {
        self.prefix_parse_fns.insert(token.to_type_name(), func);
    }

    fn register_infix(&mut self, token: TOKEN, func: infix_parse_fn) {
        self.infix_parse_fns.insert(token.to_type_name(), func);
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(self.cur_token.literal()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parser() {
        let input = r#"
          let x = 5;
          let y != 10;
          let foobar = 838383;
          "#
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{:#?}", program);
        assert_eq!(program.statements.len(), 2);
        assert_eq!(p.errors.len(), 1);
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{:#?}", program);
        assert_eq!(program.statements.len(), 1);
        assert_eq!(p.errors.len(), 0);
    }
}
