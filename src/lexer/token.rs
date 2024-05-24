use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum TOKEN {
    EOF,
    ILLEGAL(char),

    // Identifiers + literals
    IDENT(String),
    INT(i64),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,     // !
    ASTERISK, // *
    SLASH,    // /

    GT,     // >
    LT,     // <
    EQ,     // ==
    NOT_EQ, // !=

    //Delimeters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TOKEN> = {
        let mut m = HashMap::new();
        m.insert("fn", TOKEN::FUNCTION);
        m.insert("let", TOKEN::LET);
        m.insert("true", TOKEN::TRUE);
        m.insert("false", TOKEN::FALSE);
        m.insert("if", TOKEN::IF);
        m.insert("else", TOKEN::ELSE);
        m.insert("return", TOKEN::RETURN);
        m
    };
}
