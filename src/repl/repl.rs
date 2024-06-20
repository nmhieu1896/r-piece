use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

use crate::evaluator::environment::Environment;
#[allow(unused)]
use crate::{ast::ast::stringnify_stmt, lexer::lexer::Lexer, parser::parser::Parser};
use crate::{
    ast::ast::{Node, Statement},
    evaluator::eval::eval,
};

pub fn run_repl() {
    println!("Welcome to the REPL CLI. Type 'exit' to quit.");
    let env = Rc::new(RefCell::new(Environment::new()));

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

        let l = Lexer::new(&input);
        let mut p = Parser::new(l.clone());

        let program = p.parse_program();
        if program.is_err() {
            println!("{:?}", program.err().unwrap().to_string());
            continue;
        }

        // println!("{:?}", stringnify_stmt(&p.statements));
        let x = eval(
            Node::Statement(Statement::Program(program.unwrap())),
            Rc::clone(&env),
        );
        if x.is_err() {
            println!("{:?}", x.unwrap_err().to_string());
        } else {
            let str = x.unwrap().to_string();
            if str != "Null" {
                println!("{}", str);
            }
        }
    }

    println!("Exit REPL!");
}
