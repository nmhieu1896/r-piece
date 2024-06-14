use crate::{
    ast::ast::{
        BlockStatement, CallExpression, Expression, ExpressionStatement, FunctionLiteral,
        IfExpression, InfixExpression, LetStatement, PrefixExpression, Program, ReturnStatement,
        Statement,
    },
    errors::parser_errs::ParseErr,
    lexer::{lexer::Lexer, token::TOKEN},
};

use std::collections::HashMap;

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
pub struct Parser {
    l: Lexer,
    cur_token: TOKEN,
    peek_token: TOKEN,
    prefix_parse_fns: HashMap<String, PrefixParseFn>,
    infix_parse_fns: HashMap<String, InfixParseFn>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            cur_token: TOKEN::EOF,
            peek_token: TOKEN::EOF,
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        // PREFIX PARSERS
        p.register_prefix(TOKEN::IDENT("".into()), Parser::parse_identifier);
        p.register_prefix(TOKEN::NUMBER(0), Parser::parse_int_literal);
        p.register_prefix(TOKEN::TRUE, Parser::parse_boolean_literal);
        p.register_prefix(TOKEN::FALSE, Parser::parse_boolean_literal);
        p.register_prefix(TOKEN::BANG, Parser::parse_prefix_expression);
        p.register_prefix(TOKEN::MINUS, Parser::parse_prefix_expression);
        p.register_prefix(TOKEN::LPAREN, Parser::parse_group_expression);
        p.register_prefix(TOKEN::IF, Parser::parse_if_expression);
        p.register_prefix(TOKEN::FUNCTION, Parser::parse_function_literal);
        // INFIX PARSERS
        p.register_infix(TOKEN::PLUS, Parser::parse_infix_expression);
        p.register_infix(TOKEN::MINUS, Parser::parse_infix_expression);
        p.register_infix(TOKEN::SLASH, Parser::parse_infix_expression);
        p.register_infix(TOKEN::ASTERISK, Parser::parse_infix_expression);
        p.register_infix(TOKEN::EQ, Parser::parse_infix_expression);
        p.register_infix(TOKEN::NotEQ, Parser::parse_infix_expression);
        p.register_infix(TOKEN::LT, Parser::parse_infix_expression);
        p.register_infix(TOKEN::GT, Parser::parse_infix_expression);
        p.register_infix(TOKEN::LPAREN, Parser::parse_call_expression);
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

    pub fn parse_program(&mut self) -> Result<Program, ParseErr> {
        let mut program = Program { statements: vec![] };

        while self.cur_token != TOKEN::EOF {
            let stmt = self.parse_statement()?;
            program.statements.push(stmt);
            self.next_token();
        }

        return Ok(program);
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParseErr> {
        match self.cur_token {
            TOKEN::LET => Ok(self.parse_let_statement()?),
            TOKEN::RETURN => Ok(self.parse_return_statement()?),
            _ => Ok(self.parse_expression_statement()?),
        }
    }

    fn result_to_option(
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

    pub fn parse_let_statement(&mut self) -> Result<Statement, ParseErr> {
        if !self.peek_token.is_same_with(TOKEN::IDENT(String::new())) {
            return Err(ParseErr::LET("IDENT".into(), self.peek_token.clone()));
        }
        self.next_token(); // to ident token
        let name = self.cur_token.literal();

        self.next_token(); //to assign token
        if !self.cur_token.is_same_with(TOKEN::ASSIGN) {
            return Err(ParseErr::LET("ASSIGN".into(), self.cur_token.clone()));
        }

        self.next_token(); //to expression
        let value = self.parse_expression(Precedence::LOWEST)?;
        if self.peek_token.is_same_with(TOKEN::SEMICOLON) {
            self.next_token();
        }

        let stmt = LetStatement::new(name, value);

        return Ok(Statement::Let(stmt));
    }

    pub fn parse_return_statement(&mut self) -> Result<Statement, ParseErr> {
        let mut stmt = ReturnStatement::new();

        self.next_token();
        let expression = self.parse_expression(Precedence::LOWEST);
        stmt.expression = self.result_to_option(expression)?;
        if self.peek_token.is_same_with(TOKEN::SEMICOLON) {
            self.next_token();
        }
        return Ok(Statement::Return(stmt));
    }

    pub fn parse_expression_statement(&mut self) -> Result<Statement, ParseErr> {
        let mut stmt = ExpressionStatement::new(self.cur_token.clone());
        let expression = self.parse_expression(Precedence::LOWEST);
        stmt.expression = self.result_to_option(expression)?;

        if self.peek_token.is_same_with(TOKEN::SEMICOLON) {
            self.next_token();
        }

        return Ok(Statement::Expression(stmt));
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParseErr> {
        let prefix = self.prefix_parse_fns.get(&self.cur_token.to_type_name());
        if prefix.is_none() {
            return Err(ParseErr::None);
        }

        let mut left_exp = prefix.unwrap()(self)?;

        while !self.peek_token.is_same_with(TOKEN::SEMICOLON)
            && (precedence.order() < self.peek_precedence().order())
        {
            let infix = self.infix_parse_fns.get(&self.peek_token.to_type_name());
            if infix.is_none() {
                return Err(ParseErr::INFIX("INFIX".into(), self.cur_token.clone()));
            }
            self.next_token();
            //cannot use infix.unwrap()(self,left_exp) here because of mutable borrow
            //after next_token, peek_token become cur_token.
            left_exp = self
                .infix_parse_fns
                .get(&self.cur_token.to_type_name())
                .unwrap()(self, left_exp)?;
        }

        return Ok(left_exp);
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from_token(self.peek_token.clone())
    }
    fn cur_precedence(&self) -> Precedence {
        Precedence::from_token(self.cur_token.clone())
    }

    // ----------------- START EXPRESSION PARSERS ----------------------------
    fn parse_identifier(&mut self) -> Result<Expression, ParseErr> {
        Ok(Expression::Identifier(self.cur_token.literal()))
    }
    fn parse_int_literal(&mut self) -> Result<Expression, ParseErr> {
        Ok(Expression::Number(
            self.cur_token.literal().parse::<i64>().unwrap(),
        ))
    }
    fn parse_boolean_literal(&mut self) -> Result<Expression, ParseErr> {
        Ok(Expression::Bool(
            self.cur_token.literal().parse::<bool>().unwrap(),
        ))
    }
    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseErr> {
        let token = self.cur_token.clone();
        self.next_token();
        let right_exp = self.parse_expression(Precedence::PREFIX)?;
        let expression = PrefixExpression::new(token, right_exp);
        Ok(Expression::Prefix(Box::new(expression)))
    }
    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseErr> {
        let operator = self.cur_token.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        let inf_exp = InfixExpression::new(left, operator, right);
        Ok(Expression::Infix(Box::new(inf_exp)))
    }
    fn parse_group_expression(&mut self) -> Result<Expression, ParseErr> {
        self.next_token(); // to move on from "("
        let expression = self.parse_expression(Precedence::LOWEST);

        if !self.peek_token.is_same_with(TOKEN::RPAREN) {
            return Err(ParseErr::GROUP("RPAREN".into(), self.peek_token.clone()));
        }
        self.next_token(); // to move on from ")"
        return expression;
    }
    fn parse_if_expression(&mut self) -> Result<Expression, ParseErr> {
        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST)?;
        let mut expression = IfExpression::new(condition);

        if !self.peek_token.is_same_with(TOKEN::LBRACE) {
            return Err(ParseErr::IF("LBRACE".into(), self.peek_token.clone()));
        };
        self.next_token();
        expression.consequence = BlockStatement::new(self.parse_block_statement()?);

        if self.peek_token.is_same_with(TOKEN::ELSE) {
            self.next_token(); // move to ELSE

            expression.alternative = if self.peek_token.is_same_with(TOKEN::LBRACE) {
                self.next_token(); // move on from ELSE
                Some(BlockStatement::new(self.parse_block_statement()?))
            } else if self.peek_token.is_same_with(TOKEN::IF) {
                None
            } else {
                let expect = "IF or LBRACE";
                return Err(ParseErr::ELSE(expect.into(), self.peek_token.clone()));
            }
        }

        return Ok(Expression::If(Box::new(expression)));
    }
    // ----------------- END EXPRESSION PARSERS ----------------------------
    //When calling this, current token must be "{" or LBRACE
    pub fn parse_block_statement(&mut self) -> Result<Vec<Statement>, ParseErr> {
        if self.cur_token.is_same_with(TOKEN::RBRACE) {
            return Err(ParseErr::BLOCK("RBRACE".into(), self.cur_token.clone()));
        }
        let mut block_stmts = Vec::new();

        self.next_token(); // to move on from "{"
        while !self.cur_token.is_same_with(TOKEN::RBRACE) {
            if self.cur_token.is_same_with(TOKEN::EOF) {
                return Err(ParseErr::BLOCK("LBRACE".into(), self.cur_token.clone()));
            }

            let stmt = self.parse_statement()?;
            block_stmts.push(stmt);
            self.next_token();
        }

        return Ok(block_stmts);
    }
    pub fn parse_function_literal(&mut self) -> Result<Expression, ParseErr> {
        self.next_token();
        let mut function = FunctionLiteral::new(self.parse_fn_parameters()?);
        function.body = BlockStatement::new(self.parse_block_statement()?);
        return Ok(Expression::Function(Box::new(function)));
    }
    // when calling this, current token must be "(" or LPAREN
    pub fn parse_fn_parameters(&mut self) -> Result<Vec<String>, ParseErr> {
        let mut identifiers = Vec::new();
        self.next_token(); // move on from '(',
        while !self.cur_token.is_same_with(TOKEN::RPAREN) {
            match self.cur_token {
                TOKEN::IDENT(ref name) => {
                    identifiers.push(name.clone());
                    self.next_token()
                }
                _ => {
                    return Err(ParseErr::FN("IDENT".into(), self.cur_token.clone()));
                }
            }
            if self.cur_token.is_same_with(TOKEN::COMMA) {
                self.next_token()
            }
        }
        self.next_token(); // move on from (

        return Ok(identifiers);
    }
    pub fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParseErr> {
        let mut call = CallExpression::new(function);
        call.arguments = self.parse_call_args()?;
        if self.peek_token.is_same_with(TOKEN::RPAREN) {
            self.next_token();
        }
        return Ok(Expression::Call(Box::new(call)));
    }
    // call this when current token is "("
    fn parse_call_args(&mut self) -> Result<Vec<Expression>, ParseErr> {
        let mut args = vec![];

        while !self.peek_token.is_same_with(TOKEN::RPAREN) {
            if self.peek_token.is_same_with(TOKEN::EOF) {
                return Err(ParseErr::CALL("RPAREN".into(), self.peek_token.clone()));
            }
            self.next_token();
            let expression = self.parse_expression(Precedence::LOWEST);
            if let Some(exp) = self.result_to_option(expression)? {
                args.push(exp);
            }
        }

        return Ok(args);
    }
}
