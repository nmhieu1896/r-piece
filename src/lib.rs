pub mod ast;
pub mod errors;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod repl;

pub mod utils;

use lexer::lexer::Lexer;
use parser::parser::Parser;

pub fn _run() {
    let input = r#"
  let five = 5;
  let ten = 10;
  let add = fn(x, y) {
    x + y;
  };
  let result = add(five, ten);
  !-/*5;
  5 < 10 > 5;
"#;

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    println!("{:?}", p.parse_program());
}
