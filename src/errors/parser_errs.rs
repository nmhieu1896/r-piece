use std::mem::discriminant;

use thiserror::Error;

use crate::lexer::token::TOKEN;

#[derive(Debug, Error)]
pub enum ParseErr {
    // #[error("Identifier Error: {0}")]
    // IDENT(String),
    // #[error("Int Error: {0}")]
    // INT(String),
    // #[error("Return Error: {0}")]
    // RETURN(String),
    #[error("Let Error: \nExpected: {0} | got {1:?}")]
    LET(String, TOKEN),
    // #[error("Prefix Error: \nExpected: {0} | got {1:?}")]
    // PREFIX(String, TOKEN),
    #[error("Infix Error: \nExpected: {0} | got {1:?}")]
    INFIX(String, TOKEN),
    #[error("Call Error: \nExpected: {0} | got {1:?}")]
    CALL(String, TOKEN),
    #[error("Group Error: \nExpected: {0} | got {1:?}")]
    GROUP(String, TOKEN),
    #[error("If Error: \nExpected: {0} | got {1:?}")]
    IF(String, TOKEN),
    #[error("Else Error: \nExpected: {0} | got {1:?}")]
    ELSE(String, TOKEN),
    #[error("Function Error: \nExpected: {0} | got {1:?}")]
    FN(String, TOKEN),
    #[error("Block Error: \nExpected: {0} | got {1:?}")]
    BLOCK(String, TOKEN),
    #[error("None")]
    None,

    // Parse Node
    #[error("To Expression Error: \nExpected: Expression | got {0:?}")]
    ToExpression( String),
    #[error("To Statement Error: \nExpected: Statement | got {0:?}")]
    ToStatement( String),
    // Parse Expression
    #[error("To Ident Error: \nExpected: {0} | got {1:?}")]
    ToIdent(String, String),
    #[error("To Number Error: \nExpected: {0} | got {1:?}")]
    ToNum(String, String),
    #[error("To Bool Error: \nExpected: {0} | got {1:?}")]
    ToBool(String, String),
    #[error("To Prefix Error: \nExpected: {0} | got {1:?}")]
    ToPrefix(String, String),
    #[error("To Infix Error: \nExpected: {0} | got {1:?}")]
    ToInfix(String, String),
    #[error("To Call Error: \nExpected: {0} | got {1:?}")]
    ToCall(String, String),
    #[error("To If Error: \nExpected: {0} | got {1:?}")]
    ToIf(String, String),
    #[error("To Function Error: \nExpected: {0} | got {1:?}")]
    ToFn(String, String),
    // Parse Statement
    #[error("To Let Error: \nExpected: {0} | got {1:?}")]
    ToLet(String, String),
    #[error("To Return Error: \nExpected: {0} | got {1:?}")]
    ToReturn(String, String),
    #[error("To Expression Error: \nExpected: {0} | got {1:?}")]
    ToExpStmt(String, String),
    #[error("To Program Error: \nExpected: {0} | got {1:?}")]
    ToProgram(String, String),
    #[error("To Block Error: \nExpected: {0} | got {1:?}")]
    ToBlock(String, String),
}

impl ParseErr {
    #[allow(unused)]
    pub fn match_err(&self, err: ParseErr) -> bool {
        return discriminant(self) == discriminant(&err);
    }
}
