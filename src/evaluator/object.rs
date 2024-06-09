use crate::errors::coerce_errs::CoerceErr;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Object {
    Number(i64),
    String(String),
    Boolean(bool),
    Null,
    Function(fn(Vec<Object>) -> Object),
}

impl Object {
    pub fn as_int(&self) -> Result<i64, CoerceErr> {
        match self {
            Object::Number(n) => Ok(*n as i64),
            _ => Err(CoerceErr::ToInt(self.clone().into())),
        }
    }
}
