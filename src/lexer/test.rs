#[cfg(test)]
mod tests {

    use crate::{
        ast::ast::Identifier,
        lexer::{lexer::Lexer, token::TOKEN},
    };

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
          "foobar"
          "foo bar"
          "far \" boo"
          [1, 2]
        "#;

        let tokens = vec![
            TOKEN::LET,
            TOKEN::IDENT(Identifier("five".to_string())),
            TOKEN::ASSIGN,
            TOKEN::NUMBER(5),
            TOKEN::SEMICOLON,
            TOKEN::LET,
            TOKEN::IDENT(Identifier("ten".to_string())),
            TOKEN::ASSIGN,
            TOKEN::NUMBER(10),
            TOKEN::SEMICOLON,
            TOKEN::LET,
            TOKEN::IDENT(Identifier("add".to_string())),
            TOKEN::ASSIGN,
            TOKEN::FUNCTION,
            TOKEN::LPAREN,
            TOKEN::IDENT(Identifier("x".to_string())),
            TOKEN::COMMA,
            TOKEN::IDENT(Identifier("y".to_string())),
            TOKEN::RPAREN,
            TOKEN::LBRACE,
            TOKEN::IDENT(Identifier("x".to_string())),
            TOKEN::PLUS,
            TOKEN::IDENT(Identifier("y".to_string())),
            TOKEN::SEMICOLON,
            TOKEN::RBRACE,
            TOKEN::SEMICOLON,
            TOKEN::LET,
            TOKEN::IDENT(Identifier("result".to_string())),
            TOKEN::ASSIGN,
            TOKEN::IDENT(Identifier("add".to_string())),
            TOKEN::LPAREN,
            TOKEN::IDENT(Identifier("five".to_string())),
            TOKEN::COMMA,
            TOKEN::IDENT(Identifier("ten".to_string())),
            TOKEN::RPAREN,
            TOKEN::SEMICOLON,
            TOKEN::BANG,
            TOKEN::MINUS,
            TOKEN::SLASH,
            TOKEN::ASTERISK,
            TOKEN::NUMBER(6),
            TOKEN::SEMICOLON,
            TOKEN::NUMBER(7),
            TOKEN::LT,
            TOKEN::NUMBER(10),
            TOKEN::GT,
            TOKEN::NUMBER(8),
            TOKEN::SEMICOLON,
            TOKEN::IF,
            TOKEN::LPAREN,
            TOKEN::NUMBER(9),
            TOKEN::LT,
            TOKEN::NUMBER(11),
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
            TOKEN::NUMBER(13),
            TOKEN::EQ,
            TOKEN::NUMBER(13),
            TOKEN::SEMICOLON,
            TOKEN::NUMBER(14),
            TOKEN::NotEQ,
            TOKEN::NUMBER(5),
            TOKEN::SEMICOLON,
            TOKEN::STRING("foobar".to_string()),
            TOKEN::STRING("foo bar".to_string()),
            TOKEN::STRING("far \" boo".to_string()),
            TOKEN::LBRACKET,
            TOKEN::NUMBER(1),
            TOKEN::COMMA,
            TOKEN::NUMBER(2),
            TOKEN::RBRACKET,
            TOKEN::EOF,
        ];
        let mut l = Lexer::new(input);

        for token in tokens.iter() {
            assert_eq!(l.next_token(), *token);
        }
    }
}
