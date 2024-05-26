use lazy_static::lazy_static;
use std::collections::HashMap;
use std::mem::discriminant;

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

impl TOKEN {
    pub fn literal(&self) -> String {
        match self {
            TOKEN::ASSIGN => String::from("="),
            TOKEN::PLUS => String::from("+"),
            TOKEN::MINUS => String::from("-"),
            TOKEN::BANG => String::from("!"),
            TOKEN::ASTERISK => String::from("*"),
            TOKEN::SLASH => String::from("/"),
            TOKEN::GT => String::from(">"),
            TOKEN::LT => String::from("<"),
            TOKEN::EQ => String::from("=="),
            TOKEN::NOT_EQ => String::from("!="),
            TOKEN::COMMA => String::from(","),
            TOKEN::SEMICOLON => String::from(";"),
            TOKEN::LPAREN => String::from("("),
            TOKEN::RPAREN => String::from(")"),
            TOKEN::LBRACE => String::from("{"),
            TOKEN::RBRACE => String::from("}"),
            TOKEN::EOF => String::from("EOF"),
            //
            TOKEN::ILLEGAL(c) => String::from(*c),
            TOKEN::IDENT(s) => s.to_string(),
            TOKEN::INT(n) => n.to_string(),
            _ => String::from(""),
        }
    }

    pub fn is_same_with(&self, token: TOKEN) -> bool {
        // Check 2 token are same type without checking the value inside
        return discriminant(self) == discriminant(&token);
    }
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
