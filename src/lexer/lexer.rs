use std::{iter::Peekable, str::Chars};

use super::token::{KEYWORDS, TOKEN};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Lexer<'a> {
        let mut l = Lexer {
            input: input.chars().peekable(),
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        self.ch = if self.input.peek().is_none() {
            '\0'
        } else {
            self.input.next().unwrap()
        }
    }
    pub fn read_peek(&mut self) -> char {
        self.input.peek().unwrap_or(&'\0').clone()
    }

    pub fn next_token(&mut self) -> TOKEN {
        // println!("ch: {:?}", self.ch);
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
                TOKEN::NOT_EQ
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
            '\0' => TOKEN::EOF,
            c if is_letter(c) => {
                let str = self.read_identifier();
                if KEYWORDS.contains_key(str.as_str()) {
                    return KEYWORDS.get(str.as_str()).unwrap().clone();
                }
                return TOKEN::IDENT(str);
            }
            c if c.is_digit(10) => {
                return TOKEN::INT(self.read_number());
            }
            c => TOKEN::ILLEGAL(c),
        };

        self.read_char();
        return token;
    }

    pub fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_char();
        }
        return identifier;
    }

    pub fn read_number(&mut self) -> i64 {
        let mut number = String::new();
        while self.ch.is_digit(10) {
            number.push(self.ch);
            self.read_char();
        }
        return number.parse::<i64>().unwrap();
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let input = r#"
          let five = 5;
          let ten = 10;
          let add = fn(x, y) {
            x + y;
          };
          let result = add(five, ten);
          !-/*6;
          7 < 10 > 8;

          if (9 < 11) {
            return true;
          } else {
            return false;
          }

          13 == 13;
          14 != 5;
        "#
        .to_string();

        let tokens = vec![
            TOKEN::LET,
            TOKEN::IDENT("five".to_string()),
            TOKEN::ASSIGN,
            TOKEN::INT(5),
            TOKEN::SEMICOLON,
            TOKEN::LET,
            TOKEN::IDENT("ten".to_string()),
            TOKEN::ASSIGN,
            TOKEN::INT(10),
            TOKEN::SEMICOLON,
            TOKEN::LET,
            TOKEN::IDENT("add".to_string()),
            TOKEN::ASSIGN,
            TOKEN::FUNCTION,
            TOKEN::LPAREN,
            TOKEN::IDENT("x".to_string()),
            TOKEN::COMMA,
            TOKEN::IDENT("y".to_string()),
            TOKEN::RPAREN,
            TOKEN::LBRACE,
            TOKEN::IDENT("x".to_string()),
            TOKEN::PLUS,
            TOKEN::IDENT("y".to_string()),
            TOKEN::SEMICOLON,
            TOKEN::RBRACE,
            TOKEN::SEMICOLON,
            TOKEN::LET,
            TOKEN::IDENT("result".to_string()),
            TOKEN::ASSIGN,
            TOKEN::IDENT("add".to_string()),
            TOKEN::LPAREN,
            TOKEN::IDENT("five".to_string()),
            TOKEN::COMMA,
            TOKEN::IDENT("ten".to_string()),
            TOKEN::RPAREN,
            TOKEN::SEMICOLON,
            TOKEN::BANG,
            TOKEN::MINUS,
            TOKEN::SLASH,
            TOKEN::ASTERISK,
            TOKEN::INT(6),
            TOKEN::SEMICOLON,
            TOKEN::INT(7),
            TOKEN::LT,
            TOKEN::INT(10),
            TOKEN::GT,
            TOKEN::INT(8),
            TOKEN::SEMICOLON,
            TOKEN::IF,
            TOKEN::LPAREN,
            TOKEN::INT(9),
            TOKEN::LT,
            TOKEN::INT(11),
            TOKEN::RPAREN,
            TOKEN::LBRACE,
            TOKEN::RETURN,
            TOKEN::TRUE,
            TOKEN::SEMICOLON,
            TOKEN::RBRACE,
            TOKEN::ELSE,
            TOKEN::LBRACE,
            TOKEN::RETURN,
            TOKEN::FALSE,
            TOKEN::SEMICOLON,
            TOKEN::RBRACE,
            TOKEN::INT(13),
            TOKEN::EQ,
            TOKEN::INT(13),
            TOKEN::SEMICOLON,
            TOKEN::INT(14),
            TOKEN::NOT_EQ,
            TOKEN::INT(5),
            TOKEN::SEMICOLON,
            TOKEN::EOF,
        ];
        let mut l = Lexer::new(&input);

        for token in tokens.iter() {
            assert_eq!(l.next_token(), *token);
        }
    }
}
