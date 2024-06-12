#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::{
        evaluator::{environment::Environment, eval::*, object::Object},
        lexer::lexer::Lexer,
        parser::parser::Parser,
    };

    fn test_eval(input: &str) -> Result<Object, Error> {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program()?;
        let mut env = Environment::new();

        let obj = eval(&program, &mut env)?;
        return Ok(obj);
    }

    #[test]
    fn test_eval_int() {
        let test = vec![
            ("5", Object::Number(5)),
            ("-5", Object::Number(-5)),
            ("10", Object::Number(10)),
            ("-10", Object::Number(-10)),
            ("0", Object::Number(0)),
            ("-0", Object::Number(-0)),
            ("5 + 5", Object::Number(10)),
            ("5 - 5", Object::Number(0)),
            ("5 * 5", Object::Number(25)),
            ("5 / 5", Object::Number(1)),
            ("5 + 5 + 5 + 5 - 10", Object::Number(10)),
            ("2 * 2 * 2 * 2 * 2", Object::Number(32)),
            ("-50 + 100 + -50", Object::Number(0)),
            ("5 * 2 + 10", Object::Number(20)),
            ("5 + 2 * 10", Object::Number(25)),
            ("20 + 2 * -10", Object::Number(0)),
            ("50 / 2 * 2 + 10", Object::Number(60)),
            ("2 * (5 + 10)", Object::Number(30)),
            ("3 * 3 * 3 + 10", Object::Number(37)),
            ("3 * (3 * 3) + 10", Object::Number(37)),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Number(50)),
        ];
        for (input, expected) in test.into_iter() {
            let obj = test_eval(input).unwrap();
            assert_eq!(obj, expected);
        }
    }

    #[test]
    fn test_eval_bool() {
        let test = vec![
            ("!true", Object::Boolean(false)),
            ("!false", Object::Boolean(true)),
            ("!5", Object::Boolean(false)),
            ("!!true", Object::Boolean(true)),
            ("!!false", Object::Boolean(false)),
            ("!!5", Object::Boolean(true)),
            ("5 > 5", Object::Boolean(false)),
            ("5 < 5", Object::Boolean(false)),
            ("5 == 5", Object::Boolean(true)),
            ("5 != 5", Object::Boolean(false)),
            ("true == true", Object::Boolean(true)),
            ("false == false", Object::Boolean(true)),
            ("false == true", Object::Boolean(false)),
            ("true != true", Object::Boolean(false)),
            ("false != false", Object::Boolean(false)),
            ("false != true", Object::Boolean(true)),
            ("(1 < 2) == true", Object::Boolean(true)),
            ("(1 < 2) == false", Object::Boolean(false)),
            ("(1 > 2) == true", Object::Boolean(false)),
            ("(1 > 2) == false", Object::Boolean(true)),
        ];
        for (input, expected) in test.into_iter() {
            let obj = test_eval(input).unwrap();
            assert_eq!(obj, expected);
        }
    }

    #[test]
    fn test_eval_if() {
        let test = vec![
            ("if (true) { 5 }", Object::Number(5)),
            ("if (false) { 5 }", Object::Null),
            ("if (10 > 5) { 5 }", Object::Number(5)),
            ("if (5 > 10) { 5 }", Object::Null),
            ("if (5 == 5) { 5 }", Object::Number(5)),
            ("if (1 < 2) { 10 }", Object::Number(10)),
            ("if (1 > 2) { 10 }", Object::Null),
            ("if (1 > 2) { 10 } else { 20 }", Object::Number(20)),
            ("if (1 < 2) { 10 } else { 20 }", Object::Number(10)),
        ];
        for (input, expected) in test.into_iter() {
            let obj = test_eval(input).unwrap();
            println!("{:?}", obj);
            assert_eq!(obj, expected.clone());
        }
    }

    #[test]
    fn test_eval_return() {
        let test = vec![
            ("return 5", Object::Number(5)),
            ("9;return 2*5", Object::Number(10)),
            ("9;return 2*5;9", Object::Number(10)),
            ("return true", Object::Boolean(true)),
            ("return false", Object::Boolean(false)),
            (
                "#
                if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    return 1;
                }
                return 5;
            #",
                Object::Number(10),
            ),
            ("return", Object::Null),
        ];
        for (input, expected) in test.into_iter() {
            let obj = test_eval(input).unwrap();
            match obj {
                Object::Return(obj) => assert_eq!(obj.as_ref(), &expected),
                anything => assert_eq!(anything, expected),
            }
        }
    }

    #[test]
    fn test_let_stmt() {
        let test = vec![
            ("let a = 5; a", Object::Number(5)),
            ("let a = 5 * 1; let b = a; b", Object::Number(5)),
            ("let a = 5 + 0; let b = a; let c = b; c", Object::Number(5)),
            ("let a = 5; let b = a + 1; b", Object::Number(6)),
            (
                "let a = 5; let b = a + 1; let c = b + 1; c",
                Object::Number(7),
            ),
            (
                "let a = 5; let b = a + 1; let c = b + 1; let d = c * 2; d",
                Object::Number(14),
            ),
            (
                "let a = 5; let b = !a; let c = !b; c",
                Object::Boolean(true),
            ),
        ];
        for (input, expected) in test.into_iter() {
            let obj = test_eval(input).unwrap();
            assert_eq!(obj, expected);
        }
    }
}
