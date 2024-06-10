use std::mem::discriminant;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvalErr {
    #[error("Not Implemented")]
    NotImplemented,
}

impl EvalErr {
    #[allow(unused)]
    pub fn match_err(&self, err: EvalErr) -> bool {
        // Check 2 token are same type without checking the value inside
        return discriminant(self) == discriminant(&err);
    }
}
