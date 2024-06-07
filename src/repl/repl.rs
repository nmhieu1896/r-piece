use std::io::{self, Write};

use crate::{ast::ast::stringnify_stmt, lexer::lexer::Lexer, parser::parser::Parser};

pub fn run_repl() {
    println!("Welcome to the REPL CLI. Type 'exit' to quit.");

    loop {
        print!(">> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input == "exit\n" {
            break;
        }

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l.clone());

        let program = p.parse_program();
        match program {
            Ok(p) => println!("{:?}", stringnify_stmt(&p.statements)),
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Exit REPL!");
}
