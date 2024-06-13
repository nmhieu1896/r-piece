#[cfg(test)]
mod tests {
    // use std::ops::Deref;

    use std::vec;

    use crate::{ast::ast::NodeTrait, lexer::lexer::Lexer, parser::parser::Parser};

    #[test]
    fn test_parser() {
        let let_inputs = vec![
            ("let x = 1+ 5 +10;", "let", "x", "((1 + 5) + 10)"),
            ("let y = a + 10 * 2;", "let", "y", "(a + (10 * 2))"),
            ("let foobar = 838383;", "let", "foobar", "838383"),
        ];
        let return_inputs = vec![
            ("return 5+10;", "return", "(5 + 10)"),
            ("return a + 10 * 2;", "return", "(a + (10 * 2))"),
            ("return 838383;", "return", "838383"),
        ];

        for &(input, keyword, ident, val) in let_inputs.iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                assert!(false);
                return;
            }
            println!("{:#?}", program);
            let p = program.unwrap();
            let let_stmt = p.statements[0].to_let().unwrap();
            assert_eq!(let_stmt.token_literal(), keyword.to_string());
            assert_eq!(let_stmt.name.clone(), ident.to_string());
            assert_eq!(let_stmt.value.to_str(), val.to_string());
        }
        for &(input, keyword, val) in return_inputs.iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                assert!(false);
                return;
            }
            let p = program.unwrap();
            println!("{:#?}", p);
            let exp = p.statements[0].to_return().unwrap();
            assert_eq!(exp.token_literal(), keyword.to_string());
            assert_eq!(exp.expression.as_ref().unwrap().to_str(), val.to_string());
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        if program.is_err() {
            println!("{:?}", program.err().unwrap());
            assert!(false);
            return;
        };
        let p = program.unwrap();
        println!("{:#?}", p);
        let exp = p.statements[0].to_expression().unwrap();
        assert_eq!(exp.token_literal(), "foobar".to_string());
        assert_eq!(exp.to_str(), "foobar".to_string());
        assert_eq!(exp.expression.unwrap().to_ident().unwrap(), "foobar");
    }

    #[test]
    fn test_int_expression() {
        let input = "5;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        if program.is_err() {
            println!("{:?}", program.err().unwrap());
            assert!(false);
            return;
        };
        let p = program.unwrap();
        println!("{:#?}", p);
        let exp = p.statements[0].to_expression().unwrap();
        assert_eq!(exp.token_literal(), "5".to_string());
        assert_eq!(exp.to_str(), "5".to_string());
        assert_eq!(exp.expression.unwrap().to_num().unwrap(), 5);
    }
    #[test]
    fn test_prefix_expressions() {
        let inputs = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for (input, operator, value) in inputs.into_iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                assert!(false);
                return;
            };
            println!("{:#?}", program);
            let p = program.unwrap();
            let exp = p.statements[0].to_expression().unwrap();
            let prefix = exp.expression.unwrap().to_prefix().unwrap();
            assert_eq!(prefix.token.literal(), operator.to_string());
            assert_eq!(prefix.right.to_str(), value.to_string());
        }
    }

    #[test]
    fn test_infix_expressions() {
        let inputs = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for (input, left_value, operator, right_value) in inputs.into_iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                assert!(false);
                return;
            };
            let p = program.unwrap();
            let exp = p.statements[0].to_expression().unwrap();
            let infix = exp.expression.unwrap().to_infix().unwrap();
            assert_eq!(infix.token_literal(), operator.to_string());
            assert_eq!(infix.left.to_str(), left_value.to_string());
            assert_eq!(infix.right.to_str(), right_value.to_string());
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            // ("-a * b", "((-a) * b)"),
            // ("-a * 5", "((-a) * 5)"),
            // ("-5 * a", "((-5) * a)"),
            // ("!-a", "(!(-a))"),
            // ("a + b + c", "((a + b) + c)"),
            // ("a + b * c", "(a + (b * c))"),
            // ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            // ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            // ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            // ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            // (
            //     "3 + 4 * 5 == 3 * 1 + 4 * 5",
            //     "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            // ),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];

        for &(str, expected) in tests.iter() {
            let l = Lexer::new(str.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                assert!(false);
                return;
            };
            let p = program.unwrap();

            println!("p1: {:#?}", p);
            let exp = p.statements[0].to_expression().unwrap();
            assert_eq!(exp.to_str(), expected)
        }
    }

    #[test]
    fn test_group_expression() {
        let tests = vec![
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];

        for &(str1, expected) in tests.iter() {
            let l1 = Lexer::new(str1.to_string());
            let mut p1 = Parser::new(l1);
            let program = p1.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                assert!(false);
                return;
            };
            let exp = program.unwrap().statements[0].to_expression().unwrap();
            assert_eq!(exp.to_str(), expected)
        }
    }

    #[test]
    fn test_if_expression() {
        let input = r#"
        if (x < y) { 
            let x = 1;
            return x;
        } else {
           let y = 1;
           return y;
        }
        "#;

        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("p1: {:#?}", program);
        if program.is_err() {
            println!("{:?}", program.err().unwrap());
            assert!(false);
            return;
        };
        let exp = program.unwrap().statements[0].to_expression().unwrap();
        let if_exp = exp.expression.unwrap().to_if().unwrap();
        assert_eq!(if_exp.condition.to_str(), "(x < y)");
        assert_eq!(if_exp.consequence.to_str(), "{let x = 1; return x;}");
        assert_eq!(
            if_exp.alternative.clone().unwrap().to_str(),
            "{let y = 1; return y;}"
        );
        assert_eq!(
            if_exp.to_str(),
            "if (x < y) {let x = 1; return x;} else {let y = 1; return y;}"
        );
    }

    #[test]
    fn test_function_literal_parsing() {
        let v1 = vec![];
        let v2 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let inputs = vec![
            ("fn() { }", &v1, "{}"),
            ("fn(x, y, z) { x + y * z; }", &v2, "{(x + (y * z))}"),
        ];

        for &(input, parameters, block_expect) in inputs.iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                assert!(false);
                return;
            }
            let p = program.unwrap();
            println!("p: {:#?}", p);
            let exp = p.statements[0].to_expression().unwrap();
            let fn_exp = exp.expression.unwrap().to_fn().unwrap();
            assert_eq!(fn_exp.body.to_str(), block_expect);
            assert_eq!(&fn_exp.parameters, parameters);
        }
    }

    #[test]
    fn test_function_parameter_parsing() {
        let v1 = vec!["1", "(2 * 3)", "(4 + 5)"];
        let v2 = vec!["(((a + b) + ((c * d) / f)) + g)"];
        let v3 = vec!["a", "b", "1", "(2 * 3)", "(4 + 5)", "add(6, (7 * 8))"];
        let tests = vec![
            (
                "fn(a,b,c){a+b+c}(1,2*3, 4+5);",
                &v1,
                "fn(a, b, c) {((a + b) + c)}(1, (2 * 3), (4 + 5))",
            ),
            (
                "add(a + b + c * d / f + g)",
                &v2,
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                &v3,
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
        ];

        for &(input, expected_args, expected_fn_call) in tests.iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                assert!(false);
                return;
            }
            let p = program.unwrap();
            println!("p: {:#?}", p);
            let exp = p.statements[0].to_expression().unwrap();
            let call = exp.expression.unwrap().to_call().unwrap();
            assert_eq!(call.to_str(), expected_fn_call);
            for (idx, &exp) in expected_args.iter().enumerate() {
                assert_eq!(call.arguments[idx].to_str(), exp);
            }
        }
    }
}
