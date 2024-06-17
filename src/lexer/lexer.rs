use std::collections::HashMap;

use super::token::TOKEN;
// use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    position: usize,      //current position in input
    read_position: usize, //current reading position in input
    // input: Peekable<Chars<'a>>,
    ch: char,
    keywords: HashMap<String, TOKEN>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut m = HashMap::new();
        m.insert("fn".to_string(), TOKEN::FUNCTION);
        m.insert("let".to_string(), TOKEN::LET);
        m.insert("true".to_string(), TOKEN::TRUE);
        m.insert("false".to_string(), TOKEN::FALSE);
        m.insert("if".to_string(), TOKEN::IF);
        m.insert("else".to_string(), TOKEN::ELSE);
        m.insert("return".to_string(), TOKEN::RETURN);

        let mut l = Lexer {
            // input: input.chars().peekable(),
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
            keywords: m,
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        // self.ch = if self.input.peek().is_none() {
        //     '\0'
        // } else {
        //     self.input.next().unwrap()
        // }
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn read_peek(&mut self) -> char {
        // self.input.peek().unwrap_or(&'\0').clone()
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input.chars().nth(self.read_position).unwrap();
        }
    }

    pub fn next_token(&mut self) -> TOKEN {
        self.skip_white_space();
        let peek = self.read_peek();

        let token = match self.ch {
            '=' if peek == '=' => {
                self.read_char();
                TOKEN::EQ
            }
            '=' => TOKEN::ASSIGN,
            '+' => TOKEN::PLUS,
            '-' => TOKEN::MINUS,
            '!' if peek == '=' => {
                self.read_char();
                TOKEN::NotEQ
            }
            '!' => TOKEN::BANG,
            '*' => TOKEN::ASTERISK,
            '/' => TOKEN::SLASH,
            '>' => TOKEN::GT,
            '<' => TOKEN::LT,
            ',' => TOKEN::COMMA,
            ';' => TOKEN::SEMICOLON,
            '(' => TOKEN::LPAREN,
            ')' => TOKEN::RPAREN,
            '{' => TOKEN::LBRACE,
            '}' => TOKEN::RBRACE,
            '"' => {
                return TOKEN::STRING(self.read_str());
            }
            c if is_letter(c) => {
                return self.read_identifier();
            }
            c if c.is_digit(10) => {
                return TOKEN::NUMBER(self.read_number());
            }
            '\0' => TOKEN::EOF,
            c => TOKEN::ILLEGAL(c),
        };

        self.read_char();
        return token;
    }

    pub fn read_identifier(&mut self) -> TOKEN {
        let mut identifier = String::new();

        while is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_char();
        }

        if self.keywords.contains_key(identifier.as_str()) {
            return self.keywords.get(identifier.as_str()).unwrap().clone();
        }
        return TOKEN::IDENT(identifier);
    }

    pub fn read_number(&mut self) -> i64 {
        let mut number = String::new();
        while self.ch.is_digit(10) {
            number.push(self.ch);
            self.read_char();
        }
        return number.parse::<i64>().unwrap();
    }

    pub fn read_str(&mut self) -> String {
        let mut str = String::new();
        self.read_char();
        while self.ch != '"' {
            if self.ch == '\\' {
                self.read_char();
            }
            str.push(self.ch);
            self.read_char();
        }
        self.read_char();
        return str;
    }

    pub fn skip_white_space(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
