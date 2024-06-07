mod ast;
mod errors;
mod lexer;
mod parser;
mod repl;
use repl::repl::run_repl;

mod utils;

fn main() {
    defer!(println!("DONE!!!!"));
    run_repl();
}
