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

    #[error("Cannot add {0} and {1}")]
    PlusError(String, String),
    #[error("Cannot subtract {0} and {1}")]
    SubstractError(String, String),
    #[error("Cannot multiply {0} and {1}")]
    MultiplyError(String, String),
    #[error("Cannot divide {0} and {1}")]
    DivideError(String, String),
    #[error("Cannot orderly compare {0} and {1}")]
    Order(String, String),
    #[error("Cannot equally compare {0} and {1}")]
    Equal(String, String),
}

impl EvalErr {
    #[allow(unused)]
    pub fn match_err(&self, err: EvalErr) -> bool {
        return discriminant(self) == discriminant(&err);
    }
}
