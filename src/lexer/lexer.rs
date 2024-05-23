use super::token::{KEYWORDS, TOKEN};

pub struct Lexer {
    input: String,
    position: usize,      //current position in input
    read_position: usize, //current reading position in input
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> TOKEN {
        // println!("ch: {:?}", self.ch);
        let mut keep_reading = true;
        self.skip_white_space();
        let token = match self.ch {
            // SKIP white space;
            // c if c == ' ' || c == '\t' || c == '\n' || c == '\r' => {
            //     self.read_char();
            //     self.next_token()
            // }
            '=' => TOKEN::ASSIGN,
            '+' => TOKEN::PLUS,
            ',' => TOKEN::COMMA,
            ';' => TOKEN::SEMICOLON,
            '(' => TOKEN::LPAREN,
            ')' => TOKEN::RPAREN,
            '{' => TOKEN::LBRACE,
            '}' => TOKEN::RBRACE,
            '\0' => TOKEN::EOF,
            c if is_letter(c) => {
                let str = self.read_identifier();
                // println!("str: {:?}", str);
                keep_reading = false;
                if KEYWORDS.contains_key(str.as_str()) {
                    return KEYWORDS.get(str.as_str()).unwrap().clone();
                }
                TOKEN::IDENT(str)
            }
            c if c.is_digit(10) => {
                keep_reading = false;
                return TOKEN::INT(self.read_number());
            }
            c => TOKEN::ILLEGAL(c),
        };
        if keep_reading {
            self.read_char();
        }
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
        "#;

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
            TOKEN::EOF,
        ];
        let mut l = Lexer::new(input.to_string());

        for token in tokens.iter() {
            assert_eq!(l.next_token(), *token);
        }
    }
}
