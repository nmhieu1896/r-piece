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
    #[error("Let Error: Expected: {0} | got {1:?}")]
    LET(String, TOKEN),
    // #[error("Prefix Error: Expected: {0} | got {1:?}")]
    // PREFIX(String, TOKEN),
    #[error("Infix Error: Expected: {0} | got {1:?}")]
    INFIX(String, TOKEN),
    #[error("Call Error: Expected: {0} | got {1:?}")]
    CALL(String, TOKEN),
    #[error("Group Error: Expected: {0} | got {1:?}")]
    GROUP(String, TOKEN),
    #[error("If Error: Expected: {0} | got {1:?}")]
    IF(String, TOKEN),
    #[error("Else Error: Expected: {0} | got {1:?}")]
    ELSE(String, TOKEN),
    #[error("Function Error: Expected: {0} | got {1:?}")]
    FN(String, TOKEN),
    #[error("Block Error: Expected: {0} | got {1:?}")]
    BLOCK(String, TOKEN),
    #[error("None")]
    None,

    // Parse Node
    #[error("To Expression Error: Expected: Expression | got {0:?}")]
    ToExpression(String),
    #[error("To Statement Error: Expected: Statement | got {0:?}")]
    ToStatement(String),
    // Parse Expression
    #[error("To Ident Error: got {0:?}")]
    ToIdent(String),
    #[error("To String Error: got {0:?}")]
    ToString(String),
    #[error("To Number Error: got {0:?}")]
    ToNum(String),
    #[error("To Bool Error: got {0:?}")]
    ToBool(String),
    #[error("To Prefix Error: got {0:?}")]
    ToPrefix(String),
    #[error("To Infix Error: got {0:?}")]
    ToInfix(String),
    #[error("To Call Error: got {0:?}")]
    ToCall(String),
    #[error("To If Error: got {0:?}")]
    ToIf(String),
    #[error("To Function Error: got {0:?}")]
    ToFunction(String),
    // Parse Statement
    #[error("To Let Error: got {0:?}")]
    ToLet(String),
    #[error("To Return Error: got {0:?}")]
    ToReturn(String),
    #[error("To Expression Error: got {0:?}")]
    ToExpStmt(String),
    #[error("To Program Error: got {0:?}")]
    ToProgram(String),
    #[error("To Block Error: got {0:?}")]
    ToBlock(String),
}

impl ParseErr {
    #[allow(unused)]
    pub fn match_err(&self, err: ParseErr) -> bool {
        return discriminant(self) == discriminant(&err);
    }
}
