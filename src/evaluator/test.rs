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

        let obj = eval(Box::new(program))?;
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

    #[test]
    fn test_eval_int() {
        let test = vec![("5", 5), ("10", 10)];
        for &(input, expected) in test.iter() {
            let obj = test_eval(input).unwrap();
            let res = test_int_object(obj, expected);
            println!("{:?}", res);
            assert!(res.is_ok());
        }
    }
}
