#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::{
        evaluator::{eval::*, object::Object},
        lexer::lexer::Lexer,
        parser::parser::Parser,
    };

    fn test_eval(input: &str) -> Result<Object, Error> {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program()?;

        let obj = eval(&program)?;
        return Ok(obj);
    }

    fn test_int_object(obj: Object, expected: i64) -> Result<(), Error> {
        println!("OBJECT: {:?}", obj);
        let int_value = obj.as_int()?;
        if int_value == expected {
            return Ok(());
        }
        return Err(anyhow::anyhow!(
            "Expected {:?} but got {:?}",
            expected,
            int_value
        ));
    }

    fn test_bool_object(obj: Object, expected: bool) -> Result<(), Error> {
        println!("OBJECT: {:?}", obj);
        let bool_value = obj.as_bool()?;
        if bool_value == expected {
            return Ok(());
        }
        return Err(anyhow::anyhow!(
            "Expected {:?} but got {:?}",
            expected,
            bool_value
        ));
    }

    #[test]
    fn test_eval_int() {
        let test = vec![
            ("5", 5),
            ("-5", -5),
            ("10", 10),
            ("-10", -10),
            ("0", 0),
            ("-0", -0),
        ];
        for &(input, expected) in test.iter() {
            let obj = test_eval(input).unwrap();
            let res = test_int_object(obj, expected);
            println!("{:?}", res);
            assert!(res.is_ok());
        }
    }

    #[test]
    fn test_eval_bool() {
        let test = vec![("true", true), ("false", false)];
        for &(input, expected) in test.iter() {
            let obj = test_eval(input).unwrap();
            let res = test_bool_object(obj, expected);
            println!("{:?}", res);
            assert!(res.is_ok());
        }
    }
    #[test]
    fn test_bang_operator() {
        let test = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];
        for &(input, expected) in test.iter() {
            let obj = test_eval(input).unwrap();
            let res = test_bool_object(obj, expected);
            println!("{:?}", res);
            assert!(res.is_ok());
        }
    }

    #[test]
    fn test_infix_operator() {
        let test1 = vec![
            ("5 + 5", 10),
            ("5 - 5", 0),
            ("5 * 5", 25),
            ("5 / 5", 1),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];
        let test2 = vec![
            ("5 > 5", false),
            ("5 < 5", false),
            ("5 == 5", true),
            ("5 != 5", false),
            ("true == true", true),
            ("false == false", true),
            ("false == true", false),
            ("true != true", false),
            ("false != false", false),
            ("false != true", true),
        ];
        for &(input, expected) in test1.iter() {
            let obj = test_eval(input).unwrap();
            let res = test_int_object(obj, expected);
            println!("{:?}", res);
            assert!(res.is_ok());
        }
        for &(input, expected) in test2.iter() {
            let obj = test_eval(input).unwrap();
            let res = test_bool_object(obj, expected);
            println!("{:?}", res);
            assert!(res.is_ok());
        }
    }
}
