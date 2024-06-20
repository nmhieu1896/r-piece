use crate::errors::eval_errs::EvalErr;

use super::object::Object;

use once_cell::sync::Lazy;
use std::collections::HashMap;

static len: for<'a> fn(&Object<'a>) -> Result<Object<'a>, EvalErr> = |arg| match arg {
    Object::String(s) => Ok(Object::Number(s.len() as i64)),
    _ => Err(EvalErr::NotImplemented(format!(
        "{:?} is not a string",
        arg
    ))),
};
pub static Builtins: Lazy<
    HashMap<&'static str, for<'a> fn(&Object<'a>) -> Result<Object<'a>, EvalErr>>,
> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("len", len);
    m
});
