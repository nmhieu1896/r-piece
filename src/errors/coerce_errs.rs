use std::mem::discriminant;

use thiserror::Error;

use crate::evaluator::object::Object;

#[derive(Debug, Error)]
pub enum CoerceErr {
    #[error("Int Error: Can not coerce {0:?} to INT")]
    ToInt(Object),
    #[error("String Error: Can not coerce {0:?} to Identifier")]
    ToIdentifier(Object),
    #[error("Bool Error: Can not coerce {0:?} to BOOL")]
    ToBool(Object),
}

impl CoerceErr {
    #[allow(unused)]
    pub fn match_err(&self, err: CoerceErr) -> bool {
        // Check 2 token are same type without checking the value inside
        return discriminant(self) == discriminant(&err);
    }
}
