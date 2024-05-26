use std::fmt::Debug;

use crate::lexer::token::TOKEN;

trait Node {
    fn token_literal(&self) -> String;
    // fn string(&self) -> String;
}

pub trait Statement: Node + Debug {
    fn statement_node(&self);
}

trait Expression: Node + Debug {
    fn expression_node(&self);
}

#[derive(Debug)]
pub struct Program {
    // dyn keyword is for dynamic dispatch
    // this keyword is requried because "Statement" is a trait
    // and there's no hint about the size of "statements-impl-struct"
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: TOKEN,
    pub name: Option<String>, // if name is IDENT(string) => Some(String) else None
    pub value: Option<Box<dyn Expression>>,
}
impl LetStatement {
    pub fn new(token: TOKEN) -> Self {
        Self {
            token,
            name: None,
            value: None,
        }
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}
