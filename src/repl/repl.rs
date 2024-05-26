use std::io::{self, Write};

use crate::{
    lexer::{lexer::Lexer, token::TOKEN},
    parser::parser::Parser,
};

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

        let mut l = Lexer::new(input.to_string());
        let mut p = Parser::new(l.clone());

        loop {
            match l.next_token() {
                TOKEN::EOF => break,
                tk => println!("{:?}", tk),
            }
        }
        println!("{:#?}", p.parse_program());
    }

    println!("Exit REPL!");
}
