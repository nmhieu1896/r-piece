use crate::{
    ast::ast::{
        Expression, ExpressionStatement, InfixExpression, LetStatement, Node, PrefixExpression,
        Program, ReturnStatement, Statement,
    },
    lexer::{lexer::Lexer, token::TOKEN},
};

use std::collections::HashMap;

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
impl Precedence {
    pub fn from_token(token: TOKEN) -> Self {
        match token {
            TOKEN::EQ | TOKEN::NotEQ => Self::EQUALS,
            TOKEN::LT | TOKEN::GT => Self::LESSGREATER,
            TOKEN::PLUS | TOKEN::MINUS => Self::SUM,
            TOKEN::SLASH | TOKEN::ASTERISK => Self::PRODUCT,
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
pub struct Parser {
    l: Lexer,
    pub errors: Vec<String>,
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
        p.register_prefix(TOKEN::TRUE, Parser::parse_boolean_literal);
        p.register_prefix(TOKEN::FALSE, Parser::parse_boolean_literal);
        p.register_prefix(TOKEN::BANG, Parser::parse_prefix_expression);
        p.register_prefix(TOKEN::MINUS, Parser::parse_prefix_expression);
        p.register_prefix(TOKEN::LPAREN, Parser::parse_group_expression);
        //
        p.register_infix(TOKEN::PLUS, Parser::parse_infix_expression);
        p.register_infix(TOKEN::MINUS, Parser::parse_infix_expression);
        p.register_infix(TOKEN::SLASH, Parser::parse_infix_expression);
        p.register_infix(TOKEN::ASTERISK, Parser::parse_infix_expression);
        p.register_infix(TOKEN::EQ, Parser::parse_infix_expression);
        p.register_infix(TOKEN::NotEQ, Parser::parse_infix_expression);
        p.register_infix(TOKEN::LT, Parser::parse_infix_expression);
        p.register_infix(TOKEN::GT, Parser::parse_infix_expression);
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
            stmt.value = self.parse_expression_statement().unwrap().expression;
        }

        return Some(stmt);
    }

    pub fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let mut stmt = ReturnStatement::new();

        while !self.cur_token.is_same_with(TOKEN::SEMICOLON) {
            self.next_token();
            stmt.expression = self.parse_expression_statement().unwrap().expression;
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

        let mut left_exp = prefix.unwrap()(self);

        while !self.peek_token.is_same_with(TOKEN::SEMICOLON)
            && (precedence.order() < self.peek_precedence().order())
        {
            let infix = self.infix_parse_fns.get(&self.peek_token.to_type_name());
            if infix.is_none() {
                return left_exp;
            }
            self.next_token();
            //cannot use infix.unwrap()(self,left_exp) here because of mutable borrow
            //after next_token, peek_token become cur_token.
            left_exp = self
                .infix_parse_fns
                .get(&self.cur_token.to_type_name())
                .unwrap()(self, left_exp.unwrap());
        }

        return left_exp;
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from_token(self.peek_token.clone())
    }
    fn cur_precedence(&self) -> Precedence {
        Precedence::from_token(self.cur_token.clone())
    }

    // ----------------- START EXPRESSION PARSERS ----------------------------
    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(self.cur_token.literal()))
    }
    fn parse_int_literal(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(self.cur_token.literal().parse::<i64>().unwrap()))
    }
    fn parse_boolean_literal(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(self.cur_token.literal().parse::<bool>().unwrap()))
    }
    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let mut expression = PrefixExpression::new(self.cur_token.clone());
        self.next_token();
        expression.right = self.parse_expression(Precedence::PREFIX);
        Some(Box::new(expression))
    }
    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let mut inf_exp = InfixExpression::new(left, self.cur_token.clone());
        let precedence = self.cur_precedence();
        self.next_token();
        inf_exp.right = self.parse_expression(precedence);
        Some(Box::new(inf_exp))
    }
    fn parse_group_expression(&mut self) -> Option<Box<dyn Expression>> {
        self.next_token(); // to move on from "("
        let expression = self.parse_expression(Precedence::LOWEST);

        if !self.peek_token.is_same_with(TOKEN::RPAREN) {
            self.errors.push(format!(
                "Expected next token to be RPAREN, got {:?}",
                self.peek_token
            ));
            return None;
        }
        self.next_token(); // to move on from ")"
        return expression;
    }
    // ----------------- END EXPRESSION PARSERS ----------------------------
}
