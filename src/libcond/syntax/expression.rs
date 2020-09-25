use std::convert::TryFrom;

use nom::{
    branch::alt,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use super::{
    field::field,
    literal::{number, string_literal},
    operator::field_operator,
    whitespace,
};
use crate::condition::{BaseCondition, Condition};

pub fn expression<F>(i: &str) -> IResult<&str, Condition<F>>
where
    F: TryFrom<String>,
{
    map(
        delimited(
            whitespace,
            tuple((field, field_operator, alt((string_literal, number)))),
            whitespace,
        ),
        |(field_name, field_op, value)| {
            Condition::FieldCondition(BaseCondition::new(value, field_op, field_name))
        },
    )(i)
}
