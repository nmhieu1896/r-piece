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
}
