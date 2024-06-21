use crate::errors::eval_errs::EvalErr;

use super::object::Object;

use once_cell::sync::Lazy;
use std::collections::HashMap;

static GET_LEN: for<'a> fn(&Vec<Object<'a>>) -> Result<Object<'a>, EvalErr> = |arg| {
    if arg.len() != 1 {
        return Err(EvalErr::ArgsCount(1, arg.len()));
    }

    return match arg[0].clone() {
        Object::String(s) => Ok(Object::Number(s.len() as i64)),
        Object::Array(a) => Ok(Object::Number(a.as_ref().borrow().len() as i64)),
        _ => Err(EvalErr::BuiltinArgsType(
            "len".to_string(),
            "String|Array".to_string(),
            arg[0].get_type(),
        )),
    };
};

static POP: for<'a> fn(&Vec<Object<'a>>) -> Result<Object<'a>, EvalErr> = |arg| {
    if arg.len() != 1 {
        return Err(EvalErr::ArgsCount(1, arg.len()));
    }
    let arr = arg[0].to_arr(EvalErr::BuiltinArgsType(
        "pop".to_string(),
        "Array".to_string(),
        arg[0].get_type(),
    ))?;

    if arr.borrow().len() == 0 {
        return Ok(Object::Null);
    } else {
        return Ok(arr.borrow_mut().pop().unwrap());
    }
};
static PUSH: for<'a> fn(&Vec<Object<'a>>) -> Result<Object<'a>, EvalErr> = |arg| {
    if arg.len() != 2 {
        return Err(EvalErr::ArgsCount(2, arg.len()));
    }
    let arr = arg[0].to_arr(EvalErr::BuiltinArgsType(
        "push".to_string(),
        "Array".to_string(),
        arg[0].get_type(),
    ))?;
    arr.borrow_mut().push(arg[1].clone());
    Ok(arg[1].clone())
};
static POP_LEFT: for<'a> fn(&Vec<Object<'a>>) -> Result<Object<'a>, EvalErr> = |arg| {
    if arg.len() != 1 {
        return Err(EvalErr::ArgsCount(1, arg.len()));
    }
    let arr = arg[0].to_arr(EvalErr::BuiltinArgsType(
        "pop".to_string(),
        "Array".to_string(),
        arg[0].to_string(),
    ))?;

    if arr.borrow().len() == 0 {
        return Ok(Object::Null);
    } else {
        return Ok(arr.borrow_mut().remove(0));
    }
};

pub static BUILTINS: Lazy<
    HashMap<&'static str, for<'a> fn(&Vec<Object<'a>>) -> Result<Object<'a>, EvalErr>>,
> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("len", GET_LEN);
    m.insert("pop", POP);
    m.insert("pop_left", POP_LEFT);
    m.insert("push", PUSH);
    m
});
