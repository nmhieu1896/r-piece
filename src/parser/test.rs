#[cfg(test)]
mod tests {
    // use std::ops::Deref;

    use std::borrow::Borrow;

    use crate::{
        ast::ast::{
            Expression, ExpressionStatement, Identifier, InfixExpression, Node, PrefixExpression,
            Statement,
        },
        lexer::lexer::Lexer,
        parser::parser::Parser,
    };

    #[test]
    fn test_parser() {
        let input = r#"
          let x = 5;
          let y = 10;
          let foobar = 838383;
          "#
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{:#?}", program);
        assert_eq!(program.statements.len(), 3);
        assert_eq!(p.errors.len(), 0);
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{:#?}", program);
        assert_eq!(program.statements[0].token_literal(), "foobar");
        assert!(program.statements[0].as_any().is::<ExpressionStatement>());
        let stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();

        let exp = stmt.unwrap().expression.as_deref();
        assert_eq!(exp.unwrap().token_literal(), "foobar".to_string());
        assert!(exp.unwrap().as_any().is::<String>());
        assert_eq!(program.statements.len(), 1);
        assert_eq!(p.errors.len(), 0);
    }

    #[test]
    fn test_int_expression() {
        let input = "5;".to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        println!("{:#?}", program);

        let stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();
        let exp = stmt.unwrap().expression.as_deref();
        assert!(exp.unwrap().as_any().is::<i64>());
        assert_eq!(exp.unwrap().token_literal(), "5".to_string());

        assert_eq!(program.statements.len(), 1);
        assert_eq!(p.errors.len(), 0);
    }
    #[test]
    fn test_prefix_expressions() {
        let inputs = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for (input, operator, value) in inputs.into_iter() {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            println!("{:#?}", program);

            let stmt = program.statements[0]
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
            let right = prefix_exp.right.as_deref();
            assert_eq!(right.unwrap().token_literal(), value.to_string());

            assert_eq!(program.statements.len(), 1);
            assert_eq!(p.errors.len(), 0);
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
            println!("{:#?}", program);
            assert_eq!(program.statements.len(), 1);
            assert_eq!(p.errors.len(), 0);

            let stmt = program.statements[0]
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
            let right = infix_exp.right.as_deref(); // as_deref for Option<Box>
            assert_eq!(right.unwrap().token_literal(), right_value.to_string());
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

        for &(str1, expected) in tests.iter() {
            let l1 = Lexer::new(str1.to_string());
            let mut p1 = Parser::new(l1);
            let program1 = p1.parse_program();
            println!("p1: {:#?}", program1);
            let exp1 = stringnify_stmt(program1.statements);
            assert_eq!(&exp1, expected)
        }
    }

    fn stringnify_stmt(stmts: Vec<Box<dyn Statement>>) -> String {
        let mut str = String::new();
        for stmt in stmts.iter() {
            str.push_str(&stmt.to_str())
        }
        return str;
    }
}
