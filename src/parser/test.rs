#[cfg(test)]
mod tests {
    // use std::ops::Deref;

    use std::vec;

    use crate::{
        ast::ast::{
            stringnify_stmt, CallExpression, ExpressionStatement, FunctionLiteral, InfixExpression,
            LetStatement, Node, PrefixExpression, ReturnStatement,
        },
        lexer::lexer::Lexer,
        parser::parser::Parser,
    };

    #[test]
    fn test_parser() {
        let let_inputs = vec![
            ("let x = 1+ 5 +10;", "LET", "x", "((1 + 5) + 10)"),
            ("let y = a + 10 * 2;", "LET", "y", "(a + (10 * 2))"),
            ("let foobar = 838383;", "LET", "foobar", "838383"),
        ];
        let return_inputs = vec![
            ("return 5+10;", "RETURN", "(5 + 10)"),
            ("return a + 10 * 2;", "RETURN", "(a + (10 * 2))"),
            ("return 838383;", "RETURN", "838383"),
        ];

        for &(input, keyword, ident, val) in let_inputs.iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                return;
            }
            let p = program.unwrap();
            // println!("{:#?}", program);
            assert!(p.statements[0].as_any().is::<LetStatement>());
            let stmt = p.statements[0]
                .as_any()
                .downcast_ref::<LetStatement>()
                .unwrap();
            assert_eq!(stmt.token.literal(), keyword.to_string());
            assert_eq!(stmt.name.clone(), ident.to_string());
            assert_eq!(stmt.value.as_ref().to_str(), val.to_string());
        }
        for &(input, keyword, val) in return_inputs.iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                return;
            }
            let p = program.unwrap();
            println!("{:#?}", p);
            assert!(p.statements[0].as_any().is::<ReturnStatement>());
            let stmt = p.statements[0]
                .as_any()
                .downcast_ref::<ReturnStatement>()
                .unwrap();
            assert_eq!(stmt.token.literal(), keyword.to_string());
            assert_eq!(
                stmt.expression.as_deref().unwrap().to_str(),
                val.to_string()
            );
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
            return;
        };
        let p = program.unwrap();
        println!("{:#?}", p);
        assert_eq!(p.statements[0].token_literal(), "foobar");
        assert!(p.statements[0].as_any().is::<ExpressionStatement>());
        let stmt = p.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();

        let exp = stmt.unwrap().expression.as_deref();
        assert_eq!(exp.unwrap().token_literal(), "foobar".to_string());
        assert!(exp.unwrap().as_any().is::<String>());
        assert_eq!(p.statements.len(), 1);
    }

    #[test]
    fn test_int_expression() {
        let input = "5;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        if program.is_err() {
            println!("{:?}", program.err().unwrap());
            return;
        };
        let p = program.unwrap();

        println!("{:#?}", p);
        let stmt = p.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();
        let exp = stmt.unwrap().expression.as_deref();
        assert!(exp.unwrap().as_any().is::<i64>());
        assert_eq!(exp.unwrap().token_literal(), "5".to_string());

        assert_eq!(p.statements.len(), 1);
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
                return;
            };
            let p = program.unwrap();
            // println!("{:#?}", program);
            let stmt = p.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap();
            let prefix_exp = stmt
                .expression
                .as_deref()
                .unwrap()
                .as_any()
                .downcast_ref::<PrefixExpression>()
                .unwrap();
            assert_eq!(prefix_exp.token.literal(), operator.to_string());
            assert_eq!(prefix_exp.right.as_ref().token_literal(), value.to_string());

            assert_eq!(p.statements.len(), 1);
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
                return;
            };
            let p = program.unwrap();
            println!("{:#?}", p);
            assert_eq!(p.statements.len(), 1);

            let stmt = p.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap();
            println!("{:#?}", stmt);
            let infix_exp = stmt
                .expression
                .as_deref()
                .unwrap()
                .as_any()
                .downcast_ref::<InfixExpression>()
                .unwrap();
            assert_eq!(infix_exp.operator.literal(), operator.to_string());
            let left = infix_exp.left.as_ref(); //as_ref for Box
            assert_eq!(left.token_literal(), left_value.to_string());
            assert_eq!(
                infix_exp.right.as_ref().token_literal(),
                right_value.to_string()
            );
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("-a * 5", "((-a) * 5)"),
            ("-5 * a", "((-5) * a)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b * c", "(a + (b * c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
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
                return;
            };
            let p = program.unwrap();

            println!("p1: {:#?}", p);
            let exp1 = stringnify_stmt(&p.statements);
            assert_eq!(&exp1, expected)
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
            match program {
                Ok(p) => {
                    println!("p1: {:#?}", p);
                    let exp1 = stringnify_stmt(&p.statements);
                    assert_eq!(&exp1, expected)
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
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

        let l1 = Lexer::new(input.to_string());
        let mut p1 = Parser::new(l1);
        let program1 = p1.parse_program();
        println!("p1: {:#?}", program1);
        // let exp1 = stringnify_stmt(&program1.statements);
        // assert_eq!(&exp1, expected)
    }

    #[test]
    fn test_function_literal_parsing() {
        let v1 = vec![];
        let v2 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
        let inputs = vec![
            ("fn() { }", &v1, ""),
            ("fn(x, y, z) { x + y * z; }", &v2, "(x + (y * z))"),
        ];

        for &(input, parameters, block_expect) in inputs.iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            if program.is_err() {
                println!("{:?}", program.err().unwrap());
                return;
            }
            let p = program.unwrap();

            println!("p: {:#?}", p);
            let stmt = p.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap();
            let fn_stmt = stmt
                .expression
                .as_deref()
                .unwrap()
                .as_any()
                .downcast_ref::<FunctionLiteral>()
                .unwrap();
            assert_eq!(&fn_stmt.parameters, parameters);
            let block_stmt = stringnify_stmt(&fn_stmt.body.statements);
            assert_eq!(block_stmt, block_expect);
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
                return;
            }
            let p = program.unwrap();
            println!("p: {:#?}", p);
            let stmt = p.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap();
            let fn_call = stmt
                .expression
                .as_deref()
                .unwrap()
                .as_any()
                .downcast_ref::<CallExpression>()
                .unwrap();
            assert_eq!(fn_call.to_str(), expected_fn_call);
            for (idx, &exp) in expected_args.iter().enumerate() {
                assert_eq!(fn_call.arguments[idx].to_str(), exp);
            }
        }
    }
}
