mod lexer;

use lexer::lexer::Lexer;

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

    let mut l = Lexer::new(input.to_string());
    println!("{:?}", l.next_token());
}
