// use lazy_static::lazy_static;
// use phf::phf_map;
// use std::collections::HashMap;
use std::mem::discriminant;

use crate::ast::ast::{Identifier, Number};

#[derive(Debug, PartialEq, Clone)]
pub enum TOKEN {
    EOF,
    ILLEGAL(char),

    // Identifiers + literals
    IDENT(Identifier),
    NUMBER(Number),
    STRING(String),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,     // !
    ASTERISK, // *
    SLASH,    // /

    GT,    // >
    LT,    // <
    EQ,    // ==
    NotEQ, // !=

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
            TOKEN::NotEQ => String::from("!="),
            TOKEN::COMMA => String::from(","),
            TOKEN::SEMICOLON => String::from(";"),
            TOKEN::LPAREN => String::from("("),
            TOKEN::RPAREN => String::from(")"),
            TOKEN::LBRACE => String::from("{"),
            TOKEN::RBRACE => String::from("}"),
            TOKEN::EOF => String::from("EOF"),
            TOKEN::LET => String::from("LET"),
            TOKEN::RETURN => String::from("RETURN"),
            TOKEN::TRUE => String::from("true"),
            TOKEN::FALSE => String::from("false"),
            TOKEN::ELSE => String::from("else"),
            TOKEN::IF => String::from("if"),
            TOKEN::FUNCTION => String::from("fn"),
            //
            TOKEN::ILLEGAL(c) => String::from(*c),
            TOKEN::IDENT(s) => s.0.clone(),
            TOKEN::NUMBER(n) => n.to_string(),
            TOKEN::STRING(s) => s.clone(),
        }
    }
    pub fn to_type_name(&self) -> String {
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
            TOKEN::NotEQ => String::from("!="),
            TOKEN::COMMA => String::from(","),
            TOKEN::SEMICOLON => String::from(";"),
            TOKEN::LPAREN => String::from("("),
            TOKEN::RPAREN => String::from(")"),
            TOKEN::LBRACE => String::from("{"),
            TOKEN::RBRACE => String::from("}"),
            TOKEN::EOF => String::from("EOF"),
            TOKEN::LET => String::from("LET"),
            TOKEN::RETURN => String::from("RETURN"),
            TOKEN::TRUE => String::from("true"),
            TOKEN::FALSE => String::from("false"),
            TOKEN::IF => String::from("if"),
            TOKEN::ELSE => String::from("else"),
            TOKEN::FUNCTION => String::from("fn"),
            //
            TOKEN::IDENT(_) => String::from("IDENT"),
            TOKEN::NUMBER(_) => String::from("NUMBER"),
            TOKEN::STRING(_) => String::from("STRING"),
            TOKEN::ILLEGAL(_) => String::from("ILLEGAL"),
        }
    }

    pub fn is_same_with(&self, token: TOKEN) -> bool {
        // Check 2 token are same type without checking the value inside
        return discriminant(self) == discriminant(&token);
    }
}

// lazy_static! {
//     pub static ref KEYWORDS: HashMap<String, TOKEN> = {
//         let mut m = HashMap::new();
//         m.insert("fn".to_string(), TOKEN::FUNCTION);
//         m.insert("let".to_string(), TOKEN::LET);
//         m.insert("true".to_string(), TOKEN::TRUE);
//         m.insert("false".to_string(), TOKEN::FALSE);
//         m.insert("if".to_string(), TOKEN::IF);
//         m.insert("else".to_string(), TOKEN::ELSE);
//         m.insert("return".to_string(), TOKEN::RETURN);
//         m
//     };
// }

// pub static KEYWORDS: phf::Map<&'static str, TOKEN> = phf_map! {
//     "fn"=>TOKEN::FUNCTION,
//     "let"=>TOKEN::LET,
//     "true"=>TOKEN::TRUE,
//     "false"=>TOKEN::FALSE,
//     "if"=>TOKEN::IF,
//     "else"=>TOKEN::ELSE,
//     "return"=>TOKEN::RETURN,
// };
