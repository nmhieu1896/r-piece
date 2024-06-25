use std::mem::discriminant;

use thiserror::Error;

// use crate::lexer::token::TOKEN;

#[derive(Debug, Error)]
pub enum CoerceErr {
    // Parse Node
    #[error("To Expression Error: Expected: Expression | got {0}")]
    ToExpression(String),
    #[error("To Statement Error: Expected: Statement | got {0}")]
    ToStatement(String),
    // Parse Expression
    #[error("Error: {0} Cant be coerce to Ident ")]
    ToIdent(String),
    #[error("Error: {0} Cant be coerce to String ")]
    ToString(String),
    #[error("Error: {0} Cant be coerce to Number ")]
    ToNum(String),
    #[error("Error: {0} Cant be coerce to Bool ")]
    ToBool(String),
    #[error("Literal Erro got  Cant be coerce to Array {0}")]
    ToArrayLiteral(String),
    #[error("Error: {0} Cant be coerce to Prefix ")]
    ToPrefix(String),
    #[error("Error: {0} Cant be coerce to Infix ")]
    ToInfix(String),
    #[error("Error: {0} Cant be coerce to Call ")]
    ToCall(String),
    #[error("Error: {0} Cant be coerce to If ")]
    ToIf(String),
    #[error("Error: {0} Cant be coerce to Function ")]
    ToFunction(String),
    // Parse Statement
    #[error("Error: {0} Cant be coerce to Let ")]
    ToLet(String),
    #[error("Error: {0} Cant be coerce to Reassign ")]
    ToReassign(String),
    #[error("Error: {0} Cant be coerce to Return ")]
    ToReturn(String),
    #[error("Error: {0} Cant be coerce to Expression ")]
    ToExpStmt(String),
    #[error("Error: {0} Cant be coerce to Program ")]
    ToProgram(String),
    #[error("Error: {0} Cant be coerce to Block ")]
    ToBlock(String),
}

impl CoerceErr {
    #[allow(unused)]
    pub fn match_err(&self, err: CoerceErr) -> bool {
        return discriminant(self) == discriminant(&err);
    }
}
