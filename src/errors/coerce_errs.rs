use std::mem::discriminant;

use thiserror::Error;

// use crate::lexer::token::TOKEN;

#[derive(Debug, Error)]
pub enum CoerceErr {
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
    #[error("To Reassign Error: got {0:?}")]
    ToReassign(String),
    #[error("To Return Error: got {0:?}")]
    ToReturn(String),
    #[error("To Expression Error: got {0:?}")]
    ToExpStmt(String),
    #[error("To Program Error: got {0:?}")]
    ToProgram(String),
    #[error("To Block Error: got {0:?}")]
    ToBlock(String),
}

impl CoerceErr {
    #[allow(unused)]
    pub fn match_err(&self, err: CoerceErr) -> bool {
        return discriminant(self) == discriminant(&err);
    }
}
