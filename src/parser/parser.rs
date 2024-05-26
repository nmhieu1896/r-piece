use crate::{
    ast::ast::{LetStatement, Program, Statement},
    lexer::{lexer::Lexer, token::TOKEN},
};

struct Parser {
    l: Lexer,
    cur_token: TOKEN,
    peek_token: TOKEN,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            cur_token: TOKEN::EOF,
            peek_token: TOKEN::EOF,
        };
        //Read two token so current token and peek token are both set
        p.next_token();
        p.next_token();
        return p;
    }
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.cur_token != TOKEN::EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.next_token();
        }

        return program;
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token {
            TOKEN::LET => Some(Box::new(self.parse_let_statement())),
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> LetStatement {
        let mut stmt = LetStatement::new(TOKEN::LET);
        // get IDENT
        if !self.peek_token.is_same_with(TOKEN::IDENT(String::new())) {
            panic!("LET must be followed by IDENT");
        }
        stmt.name = Some(self.peek_token.literal());

        // get ASSIGN
        self.next_token();
        if !self.peek_token.is_same_with(TOKEN::ASSIGN) {
            panic!("IDENTIFIER must be followed by =");
        }

        while !self.cur_token.is_same_with(TOKEN::SEMICOLON) {
            self.next_token();
        }

        return stmt;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parser() {
        let input = r#"
          let x = 5;
          let y = 10;
          let foobar = 838383;
          "#
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{:#?}", program);
        assert_eq!(program.statements.len(), 3);
    }
}
