use std::mem::discriminant;

use thiserror::Error;

use super::{coerce_errs::CoerceErr, parser_errs::ParseErr};

#[derive(Debug, Error)]
pub enum EvalErr {
    #[error("{0}")]
    NotImplemented(String),
    #[error("Minus Prefix must be followed by a number, got {0:?}")]
    MinusPrefix(String),
    #[error("{0} ")]
    CoerceErr(#[from] CoerceErr),
    #[error("Cannot divide by zero")]
    DivideByZero,
    #[error("Identifier {0} not found")]
    IdentifierNotFound(String),
    #[error("{0}")]
    ParseErr(#[from] ParseErr),
}

impl EvalErr {
    #[allow(unused)]
    pub fn match_err(&self, err: EvalErr) -> bool {
        return discriminant(self) == discriminant(&err);
    }
}
