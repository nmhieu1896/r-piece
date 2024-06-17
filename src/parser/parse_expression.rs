use crate::{ast::ast::Expression, errors::parser_errs::ParseErr, lexer::token::TOKEN};

use super::parser::{Parser, Precedence};

pub fn parse_expression<'a>(
    parser: &mut Parser<'a>,
    precedence: Precedence,
) -> Result<Expression, ParseErr> {
    let prefix = parser
        .prefix_parse_fns
        .get(&parser.cur_token.to_type_name());
    if prefix.is_none() {
        return Err(ParseErr::None);
    }

    let mut left_exp = prefix.unwrap()(parser)?;

    while !parser.peek_token.is_same_with(TOKEN::SEMICOLON)
        && (precedence.order() < parser.peek_precedence().order())
    {
        let infix = parser
            .infix_parse_fns
            .get(&parser.peek_token.to_type_name());
        if infix.is_none() {
            return Err(ParseErr::INFIX("INFIX".into(), parser.cur_token.clone()));
        }
        parser.next_token();
        //cannot use infix.unwrap()(parser,left_exp) here because of mutable borrow
        //after next_token, peek_token become cur_token.
        left_exp = parser
            .infix_parse_fns
            .get(&parser.cur_token.to_type_name())
            .unwrap()(parser, left_exp)?;
    }

    return Ok(left_exp);
}
