use std::mem::discriminant;

use thiserror::Error;

use super::{coerce_errs::CoerceErr, parser_errs::ParseErr};

#[derive(Debug, Error)]
pub enum EvalErr {
    //Derived Errors
    #[error("{0} ")]
    CoerceErr(#[from] CoerceErr),
    #[error("{0}")]
    ParseErr(#[from] ParseErr),

    #[error("{0}")]
    NotImplemented(String),
    #[error("Minus Prefix must be followed by a number, got {0:?}")]
    MinusPrefix(String),
    #[error("Cannot divide by zero")]
    DivideByZero,
    #[error("Identifier {0} not found")]
    IdentifierNotFound(String),

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

    //
    #[error("Assign Error: {0} cant be on the left hand side of an assignment")]
    AssignLHS(String),

    //
    #[error("Variable {0} is already initialized")]
    AlreadyInitialized(String),
    //
    #[error("Indexing is only supported for array, got {0}")]
    IndexArray(String),
    #[error("Index out of bounds, index {0} is out of bounds for array of size {1}")]
    IndexOutOfBounds(i64, usize),
    //
    #[error("Object mismatch, expected {0}, got {1}")]
    CoerceObject(String, String),
    //builtin
    #[error("Expect {0} arguments, got {1}")]
    ArgsCount(usize, usize),
    #[error("Builtin({0}) expects {1} arguments, got {2}")]
    BuiltinArgsType(String, String, String),
}

impl EvalErr {
    #[allow(unused)]
    pub fn match_err(&self, err: EvalErr) -> bool {
        return discriminant(self) == discriminant(&err);
    }
}
