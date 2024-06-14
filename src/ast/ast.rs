use std::fmt::Debug;

use crate::{errors::parser_errs::ParseErr, lexer::token::TOKEN};

#[derive(Debug, Clone)]
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
    Number,
    Bool,
}

pub trait NodeTrait: Debug {
    #[allow(unused)]
    fn token_literal(&self) -> String;
    fn to_str(&self) -> String;
    fn node_type(&self) -> NodeType;
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Node {
    Expression(Expression),
    Statement(Statement),
}
impl Node {
    pub fn to_expression(&self) -> Result<Expression, ParseErr> {
        match self {
            Node::Expression(x) => Ok(x.clone()),
            x => Err(ParseErr::ToExpression(x.token_literal())),
        }
    }
    pub fn to_statement(&self) -> Result<Statement, ParseErr> {
        match self {
            Node::Statement(x) => Ok(x.clone()),
            x => Err(ParseErr::ToStatement(x.token_literal())),
        }
    }
}
impl NodeTrait for Node {
    fn node_type(&self) -> NodeType {
        match self {
            Node::Expression(x) => x.node_type(),
            Node::Statement(x) => x.node_type(),
        }
    }
    fn token_literal(&self) -> String {
        match self {
            Node::Expression(x) => x.token_literal(),
            Node::Statement(x) => x.token_literal(),
        }
    }
    fn to_str(&self) -> String {
        match self {
            Node::Expression(x) => x.to_str(),
            Node::Statement(x) => x.to_str(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Number(Number),
    Bool(bool),
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
    If(Box<IfExpression>),
    Call(Box<CallExpression>),
    Function(Box<FunctionLiteral>),
}
impl Expression {
    pub fn to_ident(&self) -> Result<Identifier, ParseErr> {
        match self {
            Expression::Identifier(x) => Ok(x.clone()),
            anything => Err(ParseErr::ToIdent(
                "Indentifier".to_string(),
                anything.token_literal(),
            )),
        }
    }
    pub fn to_num(&self) -> Result<Number, ParseErr> {
        match self {
            Expression::Number(x) => Ok(x.clone()),
            anything => Err(ParseErr::ToNum(
                "Number".to_string(),
                anything.token_literal(),
            )),
        }
    }
    pub fn to_bool(&self) -> Result<bool, ParseErr> {
        match self {
            Expression::Bool(x) => Ok(x.clone()),
            anything => Err(ParseErr::ToBool(
                "Bool".to_string(),
                anything.token_literal(),
            )),
        }
    }
    pub fn to_prefix(&self) -> Result<PrefixExpression, ParseErr> {
        match self {
            Expression::Prefix(x) => Ok(x.as_ref().clone()),
            anything => Err(ParseErr::ToPrefix(
                "Prefix".to_string(),
                anything.token_literal(),
            )),
        }
    }
    pub fn to_infix(&self) -> Result<InfixExpression, ParseErr> {
        match self {
            Expression::Infix(x) => Ok(x.as_ref().clone()),
            anything => Err(ParseErr::ToInfix(
                "Infix".to_string(),
                anything.token_literal(),
            )),
        }
    }
    pub fn to_call(&self) -> Result<CallExpression, ParseErr> {
        match self {
            Expression::Call(x) => Ok(x.as_ref().clone()),
            anything => Err(ParseErr::ToCall(
                "Call Expression".to_string(),
                anything.token_literal(),
            )),
        }
    }
    pub fn to_if(&self) -> Result<IfExpression, ParseErr> {
        match self {
            Expression::If(x) => Ok(x.as_ref().clone()),
            anything => Err(ParseErr::ToIf(
                "If Expression".to_string(),
                anything.token_literal(),
            )),
        }
    }
    pub fn to_function(&self) -> Result<FunctionLiteral, ParseErr> {
        match self {
            Expression::Function(x) => Ok(x.as_ref().clone()),
            anything => Err(ParseErr::ToFn(
                "Function Literal".to_string(),
                anything.token_literal(),
            )),
        }
    }
}
impl NodeTrait for Expression {
    fn node_type(&self) -> NodeType {
        match self {
            Expression::Identifier(_) => NodeType::Identifier,
            Expression::Number(_) => NodeType::Number,
            Expression::Bool(_) => NodeType::Bool,
            Expression::Prefix(_) => NodeType::PrefixExpression,
            Expression::Infix(_) => NodeType::InfixExpression,
            Expression::If(_) => NodeType::IfExpression,
            Expression::Call(_) => NodeType::CallExpression,
            Expression::Function(_) => NodeType::FunctionLiteral,
        }
    }
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(x) => x.token_literal(),
            Expression::Number(x) => x.token_literal(),
            Expression::Bool(x) => x.token_literal(),
            Expression::Prefix(x) => x.token_literal(),
            Expression::Infix(x) => x.token_literal(),
            Expression::If(x) => x.token_literal(),
            Expression::Call(x) => x.token_literal(),
            Expression::Function(x) => x.token_literal(),
        }
    }
    fn to_str(&self) -> String {
        match self {
            Expression::Identifier(x) => x.to_str(),
            Expression::Number(x) => x.to_str(),
            Expression::Bool(x) => x.to_str(),
            Expression::Prefix(x) => x.to_str(),
            Expression::Infix(x) => x.to_str(),
            Expression::If(x) => x.to_str(),
            Expression::Call(x) => x.to_str(),
            Expression::Function(x) => x.to_str(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Program(Program),
    Block(BlockStatement),
}
impl Statement {
    pub fn to_let(&self) -> Result<LetStatement, ParseErr> {
        match self {
            Statement::Let(x) => Ok(x.clone()),
            x => Err(ParseErr::ToLet("".to_string(), x.token_literal())),
        }
    }
    pub fn to_return(&self) -> Result<ReturnStatement, ParseErr> {
        match self {
            Statement::Return(x) => Ok(x.clone()),
            x => Err(ParseErr::ToReturn("".to_string(), x.token_literal())),
        }
    }
    pub fn to_exp_stmt(&self) -> Result<ExpressionStatement, ParseErr> {
        match self {
            Statement::Expression(x) => Ok(x.clone()),
            x => Err(ParseErr::ToExpStmt("".to_string(), x.token_literal())),
        }
    }
    pub fn to_program(&self) -> Result<Program, ParseErr> {
        match self {
            Statement::Program(x) => Ok(x.clone()),
            x => Err(ParseErr::ToProgram("".to_string(), x.token_literal())),
        }
    }
    pub fn to_block(&self) -> Result<BlockStatement, ParseErr> {
        match self {
            Statement::Block(x) => Ok(x.clone()),
            anything => Err(ParseErr::ToBlock(
                "Block".to_string(),
                anything.token_literal(),
            )),
        }
    }
}
impl NodeTrait for Statement {
    fn node_type(&self) -> NodeType {
        match self {
            Statement::Let(_) => NodeType::LetStatement,
            Statement::Return(_) => NodeType::ReturnStatement,
            Statement::Expression(_) => NodeType::ExpressionStatement,
            Statement::Program(_) => NodeType::Program,
            Statement::Block(_) => NodeType::BlockStatement,
        }
    }
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(x) => x.token_literal(),
            Statement::Return(x) => x.token_literal(),
            Statement::Expression(x) => x.token_literal(),
            Statement::Program(x) => x.token_literal(),
            Statement::Block(x) => x.token_literal(),
        }
    }
    fn to_str(&self) -> String {
        match self {
            Statement::Let(x) => x.to_str(),
            Statement::Return(x) => x.to_str(),
            Statement::Expression(x) => x.to_str(),
            Statement::Program(x) => x.to_str(),
            Statement::Block(x) => x.to_str(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl NodeTrait for Program {
    fn node_type(&self) -> NodeType {
        NodeType::Program
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

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub name: Identifier, // if name is IDENT(string) => Some(String) else None
    pub value: Expression,
}
impl LetStatement {
    pub fn new(name: Identifier, value: Expression) -> Self {
        Self { name, value }
    }
}

impl NodeTrait for LetStatement {
    fn node_type(&self) -> NodeType {
        NodeType::LetStatement
    }
    fn token_literal(&self) -> String {
        return "let".to_string();
    }
    fn to_str(&self) -> String {
        let mut str = String::from("let ");
        str.push_str(&self.name.clone());
        str.push_str(" = ");
        str.push_str(&self.value.to_str());
        str.push_str(";");

        return str;
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub expression: Option<Expression>,
}
impl ReturnStatement {
    pub fn new() -> Self {
        Self { expression: None }
    }
}

impl NodeTrait for ReturnStatement {
    fn node_type(&self) -> NodeType {
        NodeType::ReturnStatement
    }
    fn token_literal(&self) -> String {
        "return".to_string()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("");
        str.push_str(&self.token_literal());
        str.push_str(" ");
        if self.expression.is_some() {
            str.push_str(&self.expression.as_ref().unwrap().to_str());
        }
        str.push_str(";");
        return str;
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: TOKEN,
    pub expression: Option<Expression>,
}
impl ExpressionStatement {
    pub fn new(token: TOKEN) -> Self {
        Self {
            token,
            expression: None,
        }
    }
}

impl NodeTrait for ExpressionStatement {
    fn node_type(&self) -> NodeType {
        NodeType::ExpressionStatement
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }
    fn token_literal(&self) -> String {
        self.token.literal()
    }
    fn to_str(&self) -> String {
        let mut str = String::new();
        if self.expression.is_none() {
            str.push_str(&self.token.literal());
        } else {
            str.push_str(&self.expression.as_ref().unwrap().to_str());
        }
        return str;
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}
impl BlockStatement {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}
impl NodeTrait for BlockStatement {
    fn node_type(&self) -> NodeType {
        NodeType::BlockStatement
    }
    fn token_literal(&self) -> String {
        "BLOCK".to_string()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("");
        str.push_str("{");

        str.push_str(
            &self
                .statements
                .iter()
                .map(|x| x.to_str())
                .collect::<Vec<String>>()
                .join(" "),
        );
        // str.push_str(&stringnify_stmt(&self.statements));
        str.push_str("}");

        return str;
    }
}

// -------------- EXPRESSION TYPE ----------------------
//PRIMITIVE String
pub type Identifier = String;
impl NodeTrait for Identifier {
    fn node_type(&self) -> NodeType {
        NodeType::Identifier
    }
    fn to_str(&self) -> String {
        self.clone()
    }
    fn token_literal(&self) -> Identifier {
        self.clone()
    }
}
//PRIMITIVE number
pub type Number = i64;
impl NodeTrait for Number {
    fn node_type(&self) -> NodeType {
        NodeType::Number
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

impl NodeTrait for Boolean {
    fn node_type(&self) -> NodeType {
        NodeType::Bool
    }
    fn to_str(&self) -> String {
        self.to_string()
    }
    fn token_literal(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Clone)]
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

impl NodeTrait for FunctionLiteral {
    fn node_type(&self) -> NodeType {
        NodeType::FunctionLiteral
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

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: TOKEN,
    pub right: Expression,
}
impl PrefixExpression {
    pub fn new(token: TOKEN, right: Expression) -> Self {
        Self { token, right }
    }
}

impl NodeTrait for PrefixExpression {
    fn node_type(&self) -> NodeType {
        NodeType::PrefixExpression
    }
    // fn as_any(&self) -> &dyn std::any::Any {
    //     self
    // }
    fn to_str(&self) -> String {
        let mut str = String::from("(");
        str.push_str(&self.token.literal());
        str.push_str(&self.right.to_str());
        str.push(')');
        return str;
    }
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub operator: TOKEN,
    pub left: Expression,
    pub right: Expression,
}
impl InfixExpression {
    pub fn new(left: Expression, operator: TOKEN, right: Expression) -> Self {
        Self {
            left: left,
            operator,
            right,
        }
    }
}

impl NodeTrait for InfixExpression {
    fn node_type(&self) -> NodeType {
        NodeType::InfixExpression
    }

    fn token_literal(&self) -> String {
        self.operator.literal()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("(");
        str.push_str(&self.left.to_str());
        str.push_str(" ");
        str.push_str(&self.operator.literal());
        str.push_str(" ");
        str.push_str(&self.right.to_str());
        str.push(')');
        return str;
    }
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: Expression,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}
impl IfExpression {
    pub fn new(condition: Expression) -> Self {
        IfExpression {
            condition,
            consequence: BlockStatement { statements: vec![] },
            alternative: None,
        }
    }
}

impl NodeTrait for IfExpression {
    fn node_type(&self) -> NodeType {
        NodeType::IfExpression
    }
    fn token_literal(&self) -> String {
        "IF".to_string()
    }
    fn to_str(&self) -> String {
        let mut str = String::from("if ");
        str.push_str(&self.condition.to_str());
        str.push_str(" ");
        str.push_str(&self.consequence.to_str());
        if self.alternative.is_some() {
            str.push_str(" else ");
            str.push_str(&self.alternative.as_ref().unwrap().to_str());
        }
        return str;
    }
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub function: Expression, // Identifier or FunctionLiteral
    pub arguments: Vec<Expression>,
}
impl CallExpression {
    pub fn new(function: Expression) -> Self {
        Self {
            function,
            arguments: vec![],
        }
    }
}

impl NodeTrait for CallExpression {
    fn node_type(&self) -> NodeType {
        NodeType::CallExpression
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

pub fn stringnify_stmt(stmts: &Vec<Statement>) -> String {
    let mut str = String::new();
    for stmt in stmts.iter() {
        str.push_str(&stmt.to_str())
    }
    return str;
}
