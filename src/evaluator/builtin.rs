use crate::errors::eval_errs::EvalErr;

use super::object::Object;

use once_cell::sync::Lazy;
use std::collections::HashMap;

static GET_LEN: for<'a> fn(&Vec<Object<'a>>) -> Result<Object<'a>, EvalErr> = |arg| {
    if arg.len() != 1 {
        return Err(EvalErr::LenArgsCount);
    }

    return match arg[0].clone() {
        Object::String(s) => Ok(Object::Number(s.len() as i64)),
        _ => Err(EvalErr::LenArgsType),
    };
};
pub static BUILTINS: Lazy<
    HashMap<&'static str, for<'a> fn(&Vec<Object<'a>>) -> Result<Object<'a>, EvalErr>>,
> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("len", GET_LEN);
    m
});
