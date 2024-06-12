use std::mem::discriminant;

use thiserror::Error;

use crate::evaluator::object::Object;

use super::coerce_errs::CoerceErr;

#[derive(Debug, Error)]
pub enum EvalErr {
    #[error("{0}")]
    NotImplemented(String),
    #[error("Minus Prefix must be followed by a number, got {0:?}")]
    MinusPrefix(Object),
    #[error("{0} ")]
    CoerceErr(#[from] CoerceErr),
    #[error("Cannot divide by zero")]
    DivideByZero,
    #[error("Identifier {0} not found")]
    IdentifierNotFound(String),
}

impl EvalErr {
    #[allow(unused)]
    pub fn match_err(&self, err: EvalErr) -> bool {
        return discriminant(self) == discriminant(&err);
    }
}
