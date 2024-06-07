use anyhow::Error;

#[derive(Debug)]
pub struct ConversionError;

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conversion failed")
    }
}
impl std::error::Error for ConversionError {}

#[derive(Debug)]
pub enum ParseErr {
    IDENT(String),
    INT(String),
    RETURN(String),
    LET(String),
    PREFIX(String),
    INFIX(String),
    CALL(String),
    GROUP(String),
    IF(String),
    ELSE(String),
    FN(String),
    BLOCK(String),
    None,
}
impl ParseErr {
    pub fn indent(s: String) -> Error {
        Error::new(Self::IDENT(s))
    }
    pub fn int(s: String) -> Error {
        Error::new(Self::INT(s))
    }
    pub fn return_stmt(s: String) -> Error {
        Error::new(Self::RETURN(s))
    }
    pub fn let_stmt(s: String) -> Error {
        Error::new(Self::LET(s))
    }
    pub fn prefix(s: String) -> Error {
        Error::new(Self::PREFIX(s))
    }
    pub fn infix(s: String) -> Error {
        Error::new(Self::INFIX(s))
    }
    pub fn call(s: String) -> Error {
        Error::new(Self::CALL(s))
    }
    pub fn group(s: String) -> Error {
        Error::new(Self::GROUP(s))
    }
    pub fn if_exp(s: String) -> Error {
        Error::new(Self::IF(s))
    }
    pub fn else_exp(s: String) -> Error {
        Error::new(Self::ELSE(s))
    }
    pub fn function(s: String) -> Error {
        Error::new(Self::FN(s))
    }
    pub fn block(s: String) -> Error {
        Error::new(Self::BLOCK(s))
    }
    pub fn none() -> Error {
        Error::new(Self::None)
    }
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ParseErr::IDENT(s) => s,
            ParseErr::INT(s) => s,
            ParseErr::RETURN(s) => s,
            ParseErr::LET(s) => s,
            ParseErr::PREFIX(s) => s,
            ParseErr::INFIX(s) => s,
            ParseErr::CALL(s) => s,
            ParseErr::GROUP(s) => s,
            ParseErr::IF(s) => s,
            ParseErr::ELSE(s) => s,
            ParseErr::FN(s) => s,
            ParseErr::BLOCK(s) => s,
            ParseErr::None => "This is None, not Err",
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for ParseErr {}
