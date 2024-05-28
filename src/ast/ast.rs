use std::fmt::Debug;

use crate::lexer::token::TOKEN;

pub trait Node: Debug {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn as_any(&self) -> &dyn std::any::Any;
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn as_any(&self) -> &dyn std::any::Any;
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
    pub name: Option<Identifier>, // if name is IDENT(string) => Some(String) else None
    value: Option<Box<dyn Expression>>,
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn statement_node(&self) {}
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: TOKEN,
    pub expression: Option<Box<dyn Expression>>,
}
impl ExpressionStatement {
    pub fn new(token: TOKEN) -> Self {
        Self {
            token,
            expression: None,
        }
    }
}
impl Statement for ExpressionStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn statement_node(&self) {}
}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}

// -------------- EXPRESSION TYPE ----------------------
pub type Identifier = String;
impl Expression for Identifier {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn expression_node(&self) {}
}
impl Node for Identifier {
    fn token_literal(&self) -> Identifier {
        self.clone()
    }
}
pub type Integer = i64;
impl Expression for Integer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn expression_node(&self) {}
}
impl Node for Integer {
    fn token_literal(&self) -> String {
        self.to_string()
    }
}
#[derive(Debug)]
pub struct PrefixExpression {
    pub token: TOKEN,
    pub right: Option<Box<dyn Expression>>,
}
impl PrefixExpression {
    pub fn new(token: TOKEN) -> Self {
        Self { token, right: None }
    }
}
impl Expression for PrefixExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn expression_node(&self) {}
}
impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}
// -------------- EXPRESSION TYPE ----------------------
