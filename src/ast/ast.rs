use std::fmt::Debug;

use crate::lexer::token::TOKEN;

pub trait Node: Debug {
    fn token_literal(&self) -> String;
    fn to_str(&self) -> String;
}

pub trait Statement: Node {
    fn as_any(&self) -> &dyn std::any::Any;
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn as_any(&self) -> &dyn std::any::Any;
    fn expression_node(&self);
}
// pub trait AsAny {
//     fn as_any(&self) -> &dyn std::any::Any;
// }
// impl<T: Statement + Expression> AsAny for T {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }
// }

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
    fn to_str(&self) -> String {
        return String::from("PROGRAM");
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: TOKEN,
    pub name: Option<Identifier>, // if name is IDENT(string) => Some(String) else None
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn statement_node(&self) {}
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("");
        str.push_str(&self.token.literal());
        str.push_str(" ");
        str.push_str(&self.name.clone().unwrap());
        str.push_str(" = ");
        if self.value.is_some() {
            str.push_str(&self.value.as_deref().unwrap().to_str());
        }
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn statement_node(&self) {}
}
impl Node for ReturnStatement {
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn statement_node(&self) {}
}
impl Node for ExpressionStatement {
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
    fn token_literal(&self) -> String {
        "{}".to_string()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("if ");
        str.push_str(" {\n");
        str.push_str(&stringnify_stmt(&self.statements));
        str.push_str("\n}");

        return str;
    }
}

// -------------- EXPRESSION TYPE ----------------------
//PRIMITIVE String
pub type Identifier = String;
impl Expression for Identifier {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn expression_node(&self) {}
}
impl Node for Identifier {
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn expression_node(&self) {}
}
impl Node for Integer {
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn expression_node(&self) {}
}
impl Node for Boolean {
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn expression_node(&self) {}
}
impl Node for FunctionLiteral {
    fn to_str(&self) -> String {
        let mut str = String::from("fn (");

        let vec = self
            .parameters
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        str.push_str(&vec.join(", "));
        str.push_str(") {\n");
        str.push_str(&stringnify_stmt(&self.body.statements));
        str.push_str("\n}");

        str
    }
    fn token_literal(&self) -> String {
        TOKEN::FUNCTION.literal()
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
    fn to_str(&self) -> String {
        let mut str = String::from("(");
        str.push_str(&self.token.literal());
        str.push_str(&self.right.as_deref().unwrap().to_str());
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
    pub right: Option<Box<dyn Expression>>,
}
impl InfixExpression {
    pub fn new(left: Box<dyn Expression>, operator: TOKEN) -> Self {
        Self {
            left: left,
            operator,
            right: None,
        }
    }
}
impl Expression for InfixExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn expression_node(&self) {}
}
impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.operator.literal()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("(");
        str.push_str(&self.left.as_ref().to_str());
        str.push_str(" ");
        str.push_str(&self.operator.literal());
        str.push_str(" ");
        str.push_str(&self.right.as_deref().unwrap().to_str());
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
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn expression_node(&self) {}
}
impl Node for IfExpression {
    fn token_literal(&self) -> String {
        "IF".to_string()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("if ");
        str.push_str(&self.condition.to_str());
        str.push_str(&self.consequence.to_str());
        // str.push_str(" {\n");
        // str.push_str(&stringnify_stmt(&self.consequence.statements));
        // str.push_str("\n}");

        if self.alternative.is_some() {
            str.push_str(&self.alternative.as_ref().unwrap().to_str());
            // str.push_str("else {\n");
            // str.push_str(&stringnify_stmt(
            //     &self.alternative.as_ref().unwrap().statements,
            // ));
            // str.push_str("\n}");
        }
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
