use std::{fmt::Debug, marker::Unsize};

use crate::lexer::token::TOKEN;

#[derive(Debug)]
pub enum NodeType {
    Program,
    LetStatement,
    ReturnStatement,
    ExpressionStatement,
    BlockStatement,
    FunctionLiteral,
    PrefixExpression,
    InfixExpression,
    IfExpression,
    CallExpression,
    //
    Identifier,
    Int,
    Bool,
}

pub trait Node: Debug {
    #[allow(unused)]
    fn as_any(&self) -> &dyn std::any::Any;
    fn token_literal(&self) -> String;
    fn to_str(&self) -> String;
    fn node_type(&self) -> NodeType;
}
pub fn upcast_trait<Dyn: ?Sized + Unsize<dyn Node>>(bar: &Dyn) -> &dyn Node {
    bar
}
pub trait Statement: Node {
    // #[allow(unused)]
    // fn as_any(&self) -> &dyn std::any::Any;
    #[allow(unused)]
    fn statement_node(&self);
}

pub trait Expression: Node {
    // #[allow(unused)]
    // fn as_any(&self) -> &dyn std::any::Any;
    #[allow(unused)]
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
    fn node_type(&self) -> NodeType {
        NodeType::Program
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
    fn to_str(&self) -> String {
        return String::from("PROGRAM");
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: TOKEN,
    pub name: Identifier, // if name is IDENT(string) => Some(String) else None
    pub value: Box<dyn Expression>,
}
impl LetStatement {
    pub fn new(token: TOKEN, name: Identifier, value: Box<dyn Expression>) -> Self {
        Self { token, name, value }
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}
impl Node for LetStatement {
    fn node_type(&self) -> NodeType {
        NodeType::LetStatement
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("");
        str.push_str(&self.token.literal());
        str.push_str(" ");
        str.push_str(&self.name.clone());
        str.push_str(" = ");
        str.push_str(&self.value.as_ref().to_str());

        return str;
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: TOKEN,
    pub expression: Option<Box<dyn Expression>>,
}
impl ReturnStatement {
    pub fn new() -> Self {
        Self {
            token: TOKEN::RETURN,
            expression: None,
        }
    }
}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}
impl Node for ReturnStatement {
    fn node_type(&self) -> NodeType {
        NodeType::ReturnStatement
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("");
        str.push_str(&self.token.literal());
        str.push_str(" ");
        if self.expression.is_some() {
            str.push_str(&self.expression.as_deref().unwrap().to_str());
        }
        return str;
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
    fn statement_node(&self) {}
}
impl Node for ExpressionStatement {
    fn node_type(&self) -> NodeType {
        NodeType::ExpressionStatement
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    fn to_str(&self) -> String {
        let mut str = String::new();
        if self.expression.is_none() {
            str.push_str(&self.token.literal());
        } else {
            str.push_str(&self.expression.as_deref().unwrap().to_str());
        }
        return str;
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
}
impl BlockStatement {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Self { statements }
    }
}
impl Node for BlockStatement {
    fn node_type(&self) -> NodeType {
        NodeType::BlockStatement
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn token_literal(&self) -> String {
        "{}".to_string()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("if ");
        str.push_str(" {");
        str.push_str(&stringnify_stmt(&self.statements));
        str.push_str("}");

        return str;
    }
}

// -------------- EXPRESSION TYPE ----------------------
//PRIMITIVE String
pub type Identifier = String;
impl Expression for Identifier {
    fn expression_node(&self) {}
}
impl Node for Identifier {
    fn node_type(&self) -> NodeType {
        NodeType::Identifier
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn to_str(&self) -> String {
        self.clone()
    }
    fn token_literal(&self) -> Identifier {
        self.clone()
    }
}
//PRIMITIVE number
pub type Integer = i64;
impl Expression for Integer {
    fn expression_node(&self) {}
}
impl Node for Integer {
    fn node_type(&self) -> NodeType {
        NodeType::Int
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn token_literal(&self) -> String {
        self.to_string()
    }
}
//PRIMITIVE Boolean
pub type Boolean = bool;
impl Expression for Boolean {
    fn expression_node(&self) {}
}
impl Node for Boolean {
    fn node_type(&self) -> NodeType {
        NodeType::Bool
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn token_literal(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug)]
pub struct FunctionLiteral {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}
impl FunctionLiteral {
    pub fn new(parameters: Vec<Identifier>) -> Self {
        Self {
            parameters: parameters,
            body: BlockStatement { statements: vec![] },
        }
    }
}
impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}
impl Node for FunctionLiteral {
    fn node_type(&self) -> NodeType {
        NodeType::FunctionLiteral
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn to_str(&self) -> String {
        let mut str = String::from("fn(");

        let vec = self
            .parameters
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        str.push_str(&vec.join(", "));
        str.push_str(") {");
        str.push_str(&stringnify_stmt(&self.body.statements));
        str.push_str("}");

        str
    }
    fn token_literal(&self) -> String {
        TOKEN::FUNCTION.literal()
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: TOKEN,
    pub right: Box<dyn Expression>,
}
impl PrefixExpression {
    pub fn new(token: TOKEN, right: Box<dyn Expression>) -> Self {
        Self { token, right }
    }
}
impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}
impl Node for PrefixExpression {
    fn node_type(&self) -> NodeType {
        NodeType::PrefixExpression
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn to_str(&self) -> String {
        let mut str = String::from("(");
        str.push_str(&self.token.literal());
        str.push_str(&self.right.as_ref().to_str());
        str.push(')');
        return str;
    }
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub operator: TOKEN,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}
impl InfixExpression {
    pub fn new(left: Box<dyn Expression>, operator: TOKEN, right: Box<dyn Expression>) -> Self {
        Self {
            left: left,
            operator,
            right,
        }
    }
}
impl Expression for InfixExpression {
    fn expression_node(&self) {}
}
impl Node for InfixExpression {
    fn node_type(&self) -> NodeType {
        NodeType::InfixExpression
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn token_literal(&self) -> String {
        self.operator.literal()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("(");
        str.push_str(&self.left.as_ref().to_str());
        str.push_str(" ");
        str.push_str(&self.operator.literal());
        str.push_str(" ");
        str.push_str(&self.right.as_ref().to_str());
        str.push(')');
        return str;
    }
}

#[derive(Debug)]
pub struct IfExpression {
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}
impl IfExpression {
    pub fn new(condition: Box<dyn Expression>) -> Self {
        IfExpression {
            condition,
            consequence: BlockStatement { statements: vec![] },
            alternative: None,
        }
    }
}
impl Expression for IfExpression {
    fn expression_node(&self) {}
}
impl Node for IfExpression {
    fn node_type(&self) -> NodeType {
        NodeType::IfExpression
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn token_literal(&self) -> String {
        "IF".to_string()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("if ");
        str.push_str(&self.condition.to_str());
        str.push_str(&self.consequence.to_str());

        if self.alternative.is_some() {
            str.push_str(&self.alternative.as_ref().unwrap().to_str());
        }
        return str;
    }
}

#[derive(Debug)]
pub struct CallExpression {
    pub function: Box<dyn Expression>, // Identifier or FunctionLiteral
    pub arguments: Vec<Box<dyn Expression>>,
}
impl CallExpression {
    pub fn new(function: Box<dyn Expression>) -> Self {
        Self {
            function,
            arguments: vec![],
        }
    }
}
impl Expression for CallExpression {
    fn expression_node(&self) {}
}
impl Node for CallExpression {
    fn node_type(&self) -> NodeType {
        NodeType::CallExpression
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn token_literal(&self) -> String {
        "CALL".to_string()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("");
        str.push_str(&self.function.to_str());
        str.push('(');
        str.push_str(
            &self
                .arguments
                .iter()
                .map(|x| x.to_str())
                .collect::<Vec<String>>()
                .join(", "),
        );
        str.push(')');
        return str;
    }
}
// -------------- EXPRESSION TYPE ----------------------

pub fn stringnify_stmt(stmts: &Vec<Box<dyn Statement>>) -> String {
    let mut str = String::new();
    for stmt in stmts.iter() {
        str.push_str(&stmt.to_str())
    }
    return str;
}
