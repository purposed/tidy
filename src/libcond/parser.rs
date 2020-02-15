use std::convert::TryFrom;

use logos::{Lexer, Logos};

use rood::{Cause, CausedResult, Error};

use crate::condition::{BaseCondition, BoolOperator, Condition, FieldOperator, NestedCondition};
use crate::lexer::Token;

struct Parser<'a> {
    lexer: Lexer<Token, &'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        Parser {
            lexer: Token::lexer(source),
        }
    }

    fn expect(&mut self, tok_type: Token) -> CausedResult<String> {
        if self.lexer.token == tok_type {
            let slice_val = self.lexer.slice();
            self.lexer.advance();
            return Ok(String::from(slice_val));
        }

        Err(Error::new(
            Cause::GeneralError(String::from("ParseError")),
            &format!("Expected {:?}, got {:?}", tok_type, self.lexer.token),
        ))
    }

    fn current_token(&self) -> CausedResult<(&Token, &str)> {
        if self.lexer.token == Token::End || self.lexer.token == Token::Error {
            return Err(Error::new(
                Cause::GeneralError(String::from("ParseError")),
                "Unexpected end of input",
            ));
        }
        Ok((&self.lexer.token, self.lexer.slice()))
    }

    pub fn parse_expression<F>(&mut self) -> CausedResult<Condition<F>>
    where
        F: TryFrom<String>,
    {
        let field_name = self.expect(Token::Field)?;
        let field = match F::try_from(String::from(field_name.trim_start_matches('@'))) {
            Ok(a) => Ok(a),
            Err(_) => Err(Error::new(
                Cause::InvalidData,
                &format!("Unknown field: '{}'", field_name),
            )),
        }?;
        let field_op = FieldOperator::try_from(self.expect(Token::FieldOperator)?)?;

        let value = match self.current_token()? {
            (Token::Literal, v) | (Token::Number, v) => Ok(String::from(v.trim_matches('"'))),
            _ => Err(Error::new(
                Cause::GeneralError(String::from("ParserError")),
                "Number or Literal expected",
            )),
        }?;
        self.lexer.advance();
        Ok(Condition::FieldCondition(BaseCondition::new(
            value, field_op, field,
        )))
    }

    pub fn parse_nested_condition<F>(&mut self) -> CausedResult<Condition<F>>
    where
        F: TryFrom<String>,
    {
        let left_member = self.parse_condition()?;
        let operator = BoolOperator::try_from(self.expect(Token::BoolOperator)?)?;
        let right_member = self.parse_condition()?;
        self.expect(Token::ParenClose)?;

        Ok(Condition::BoolCondition(Box::from(NestedCondition::new(
            left_member,
            operator,
            right_member,
        ))))
    }

    pub fn parse_condition<F>(&mut self) -> CausedResult<Condition<F>>
    where
        F: TryFrom<String>,
    {
        // Basic grammar
        // EXPR => FIELD FIELD_OP VAL
        // CONDITION => EXPR
        //             | ( CONDITION BOOL_OP CONDITION)
        match self.current_token()? {
            (Token::ParenOpen, _) => {
                self.lexer.advance();
                self.parse_nested_condition()
            }
            _ => self.parse_expression(),
        }
    }
}

pub fn parse<F>(src: &str) -> CausedResult<Condition<F>>
where
    F: TryFrom<String>,
{
    let mut p = Parser::new(src);
    p.parse_condition()
}
