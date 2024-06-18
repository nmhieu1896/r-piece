use std::mem::discriminant;

use thiserror::Error;

use crate::lexer::token::TOKEN;

use super::coerce_errs::CoerceErr;

#[derive(Debug, Error)]
pub enum ParseErr {
    //Derived Errors
    #[error("{0} ")]
    CoerceErr(#[from] CoerceErr),
    //
    #[error("Let Error: Expected: {0} | got {1:?}")]
    LET(String, TOKEN),
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
}

impl ParseErr {
    #[allow(unused)]
    pub fn match_err(&self, err: ParseErr) -> bool {
        return discriminant(self) == discriminant(&err);
    }
}
