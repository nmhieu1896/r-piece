mod ast;
mod errors;
mod evaluator;
mod lexer;
mod parser;
mod repl;
use repl::repl::run_repl;

mod utils;

fn main() {
    run_repl();
}
