use std::convert::TryFrom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use super::{expression::expression, operator::bool_operator, whitespace};
use crate::condition::{Condition, NestedCondition};

pub fn nested_condition<F>(i: &str) -> IResult<&str, Condition<F>>
where
    F: TryFrom<String>,
{
    map(
        delimited(
            whitespace,
            delimited(
                tag("("),
                tuple((condition, bool_operator, condition)),
                tag(")"),
            ),
            whitespace,
        ),
        |(left_member, bool_op, right_member)| {
            Condition::BoolCondition(Box::from(NestedCondition::new(
                left_member,
                bool_op,
                right_member,
            )))
        },
    )(i)
}

pub fn condition<F>(i: &str) -> IResult<&str, Condition<F>>
where
    F: TryFrom<String>,
{
    alt((nested_condition, expression))(i)
}
