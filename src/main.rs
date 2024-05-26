mod ast;
mod lexer;
mod parser;
mod repl;
use repl::repl::run_repl;

fn main() {
    run_repl();
}
