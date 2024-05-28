use std::collections::HashMap;

use crate::{
    ast::ast::{
        Expression, ExpressionStatement, LetStatement, Node, PrefixExpression, Program, Statement,
    },
    lexer::{lexer::Lexer, token::TOKEN},
};

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;
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
    prefix_parse_fns: HashMap<String, PrefixParseFn>,
    infix_parse_fns: HashMap<String, InfixParseFn>,
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
        p.register_prefix(TOKEN::IDENT(String::new()), Parser::parse_identifier);
        p.register_prefix(TOKEN::INT(0), Parser::parse_int_literal);
        p.register_prefix(TOKEN::BANG, Parser::parse_prefix_expression);
        p.register_prefix(TOKEN::MINUS, Parser::parse_prefix_expression);

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
            // return None;
        }
        stmt.name = Some(self.peek_token.literal());

        // get ASSIGN
        self.next_token();
        if !self.peek_token.is_same_with(TOKEN::ASSIGN) {
            self.errors.push(format!(
                "Expected next token to be ASSIGN, got {:?}",
                self.peek_token
            ));
            // return None;
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

    fn register_prefix(&mut self, token: TOKEN, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token.to_type_name(), func);
    }

    fn register_infix(&mut self, token: TOKEN, func: InfixParseFn) {
        self.infix_parse_fns.insert(token.to_type_name(), func);
    }

    // TOKEN Parsers
    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(self.cur_token.literal()))
    }
    fn parse_int_literal(&mut self) -> Option<Box<dyn Expression>> {
        match self.cur_token {
            TOKEN::INT(i) => Some(Box::new(i)),
            _ => {
                self.errors.push(format!(
                    "parse_int_literal: Expected next token to be INT, got {:?}",
                    self.cur_token
                ));
                return None;
            }
        }
    }
    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let mut expression = PrefixExpression::new(self.cur_token.clone());
        self.next_token();
        expression.right = self.parse_expression(Precedence::PREFIX);
        Some(Box::new(expression))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = r#"
          let x = 5;
          let y = 10;
          let foobar = 838383;
          "#
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{:#?}", program);
        assert_eq!(program.statements.len(), 3);
        assert_eq!(p.errors.len(), 0);
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{:#?}", program);
        assert_eq!(program.statements[0].token_literal(), "foobar");
        assert!(program.statements[0].as_any().is::<ExpressionStatement>());
        let stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();

        let exp = stmt.unwrap().expression.as_deref();
        assert_eq!(exp.unwrap().token_literal(), "foobar".to_string());
        assert!(exp.unwrap().as_any().is::<String>());
        assert_eq!(program.statements.len(), 1);
        assert_eq!(p.errors.len(), 0);
    }

    #[test]
    fn test_int_expression() {
        let input = "5;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{:#?}", program);

        let stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();
        let exp = stmt.unwrap().expression.as_deref();
        assert!(exp.unwrap().as_any().is::<i64>());
        assert_eq!(exp.unwrap().token_literal(), "5".to_string());

        assert_eq!(program.statements.len(), 1);
        assert_eq!(p.errors.len(), 0);
    }
    #[test]
    fn test_prefix_expressions() {
        let inputs = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for (input, operator, value) in inputs.into_iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            println!("{:#?}", program);

            let stmt = program.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap();
            let prefix_exp = stmt
                .expression
                .as_deref()
                .unwrap()
                .as_any()
                .downcast_ref::<PrefixExpression>()
                .unwrap();
            assert_eq!(prefix_exp.token.literal(), operator.to_string());
            let right = prefix_exp.right.as_deref();
            assert_eq!(right.unwrap().token_literal(), value.to_string());

            assert_eq!(program.statements.len(), 1);
            assert_eq!(p.errors.len(), 0);
        }
    }
}
