#[cfg(test)]
mod tests {

    use std::{cell::RefCell, rc::Rc};

    use crate::{
        ast::ast::{Node, NodeTrait, Statement},
        errors::eval_errs::EvalErr,
        evaluator::{environment::Environment, eval::*, object::Object},
        lexer::lexer::Lexer,
        parser::parser::Parser,
    };

    fn test_eval(input: &str) -> Result<Object, EvalErr> {
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        let env = Rc::new(RefCell::new(Environment::new()));
        println!("{:#?}", program);
        return eval(
            Node::Statement(Statement::Program(program)),
            Rc::clone(&env),
        );
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
            (
                "if (1 > 2) { 10 } else if (2 > 1) { 15 } else { 20 }",
                Object::Number(15),
            ),
            (
                "if (1 > 2) { 10 } else if (2 > 3) { 15 } else { 20 }",
                Object::Number(20),
            ),
            (
                r#"
                if ( 10 == 2*4 ) { 
                    "10 == 2*4"
                } else if (10 == 2*5) { 
                    "10 == 2*5"
                } else { 
                    "I dont know"
                }"#,
                Object::String("10 == 2*5".into()),
            ),
            (
                r#"
                if ( 10 == 2*4 ) { 
                    "10 == 2*4"
                } else if (10 < 2*5) { 
                    "10 < 2*5"
                } else { 
                    "I dont know"
                }"#,
                Object::String("I dont know".into()),
            ),
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
                "
                if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    return 1;
                }
                return 5;
            ",
                Object::Number(10),
            ),
            ("return", Object::Null),
        ];
        for (input, expected) in test.into_iter() {
            let obj = test_eval(input).unwrap();
            match obj {
                Object::Return(obj) => assert_eq!(obj.as_ref(), &expected),
                anything => assert_eq!(anything, Object::Return(Box::new(expected))),
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
            //Re assign
            ("let a = 5; a = 10; a", Object::Number(10)),
            ("let a = 5; a = 10; let b = a; b", Object::Number(10)),
            (
                "let a = 5; a = 10; let b = a; let c = b; c",
                Object::Number(10),
            ),
            (
                "let a = 5; fn(newA) {a = newA}(100); a",
                Object::Number(100),
            ),
        ];
        for (input, expected) in test.into_iter() {
            let obj = test_eval(input).unwrap();
            assert_eq!(obj, expected);
        }

        let test = vec!["b = 1", "let a = 5; let a = 10", "let a = 5; b = 10"];
        for input in test.into_iter() {
            let obj = test_eval(input);
            assert!(obj.is_err());
        }
    }

    #[test]
    fn test_function() {
        let args1 = vec!["a", "b"];
        let args2 = vec!["a", "b", "c"];
        let args3 = vec!["a", "b", "c", "d"];
        let test = vec![
            ("fn(a, b) { return a + b; }; ", args1, "{return (a + b);}"),
            (
                "fn(a, b, c) { return a + b + c; };",
                args2,
                "{return ((a + b) + c);}",
            ),
            (
                "fn(a, b, c, d) { return a + b + c +d; }; ",
                args3,
                "{return (((a + b) + c) + d);}",
            ),
        ];
        for (input, args, expected) in test.into_iter() {
            let obj = test_eval(input).unwrap();
            println!("{:?}", obj);
            match obj {
                Object::Function(f) => {
                    assert_eq!(
                        f.params
                            .iter()
                            .map(|x| x.0.clone())
                            .collect::<Vec<String>>(),
                        args
                    );
                    assert_eq!(f.body.to_str(), expected);
                }
                anything => panic!("{:?}", anything),
            }
        }
    }

    #[test]
    fn test_call_fn() {
        let tests = vec![
            (
                "let x = fn(a, b) { return a + b; }; x(5, 5)",
                Object::Number(10),
            ),
            (
                "let x = fn(a, b, c) { return a + b + c; }; x(5, 5, 5)",
                Object::Number(15),
            ),
            (
                "let y = fn(a, b, c, d) { return a + b + c * d; }; y(5, 5, 5, 5)",
                Object::Number(35),
            ),
            ("fn(x) { x; }(5)", Object::Number(5)),
            ("fn(x) { x; }(true)", Object::Boolean(true)),
            ("fn(x) { x; }(5>3)", Object::Boolean(true)),
            ("fn(x) { x > 3; }( 10 )", Object::Boolean(true)),
            ("fn(x) { x < 3; }( 10 )", Object::Boolean(false)),
            ("let x = 10 ;fn() { x < 3; }()", Object::Boolean(false)),
            ("let x = 10 ;fn() { x > 3; }()", Object::Boolean(true)),
            (
                "
            let newAdder = fn(x) {
                fn(y) { x + y };
            };
            let addTwo = newAdder(2);
            addTwo(2)
            ",
                Object::Number(4),
            ),
        ];
        for (input, expected) in tests.into_iter() {
            let obj = test_eval(input).unwrap();
            assert_eq!(obj, expected);
        }
    }

    #[test]
    fn test_builtin_functions() {
        let tests = vec![
            (r#"len("")"#, Object::Number(0)),
            (r#"len("hello")"#, Object::Number(5)),
            (r#"len("hello world")"#, Object::Number(11)),
        ];
        for (input, expected) in tests.into_iter() {
            let obj = test_eval(input).unwrap();
            assert_eq!(obj, expected);
        }
    }

    #[test]
    fn test_array_index() {
        let tests = vec![
            ("[1, 2, 3][0]", Object::Number(1)),
            ("[1, 2, 3, 4, 5][1]", Object::Number(2)),
            ("let my_array = [0,2,3];my_array[0]", Object::Number(0)),
            ("[1+2,3*5,[1,2*3]][0]", Object::Number(3)),
            (
                r#"
                let a = 10;
                let b = 20;
                let func = fn(x) { return [x, x*a, x*b] };
                let c = func(2);
                c[0] + c[1] + c[2]
            "#,
                Object::Number(62),
            ),
        ];
        for (input, expected) in tests.into_iter() {
            let obj = test_eval(input).unwrap();
            assert_eq!(obj, expected);
        }
    }
}
