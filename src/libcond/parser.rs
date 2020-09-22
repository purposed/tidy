use std::convert::TryFrom;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alphanumeric1, anychar, char, digit1},
    combinator::{all_consuming, map, map_res, opt, recognize, verify},
    multi::{many0, many0_count},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use snafu::Snafu;

use crate::condition::{BaseCondition, BoolOperator, Condition, FieldOperator, NestedCondition};


fn literal(i: &str) -> IResult<&str, String> {
    map(delimited(whitespace, many0(anychar), whitespace), |c| {
        c.into_iter().collect()
    })(i)
}

fn number(i: &str) -> IResult<&str, String> {
    map(digit1, String::from)(i)
}

pub fn identifier(s: &str) -> IResult<&str, &str> {
    recognize(all_consuming(pair(
        verify(anychar, |&c| c.is_lowercase()),
        many0_count(preceded(opt(char('_')), alphanumeric1)),
    )))(s)
}

fn field<F: TryFrom<String>>(i: &str) -> IResult<&str, F> {
    map_res(preceded(tag("@"), identifier), |c| {
        F::try_from(String::from(c))
    })(i)
}

fn expression<F>(i: &str) -> IResult<&str, Condition<F>>
where
    F: TryFrom<String>,
{
    map(
        delimited(
            whitespace,
            tuple((field, field_operator, alt((literal, number)))),
            whitespace,
        ),
        |(field_name, field_op, value)| {
            Condition::FieldCondition(BaseCondition::new(value, field_op, field_name))
        },
    )(i)
}

fn nested_condition<F>(i: &str) -> IResult<&str, Condition<F>>
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

fn condition<F>(i: &str) -> IResult<&str, Condition<F>>
where
    F: TryFrom<String>,
{
    alt((nested_condition, expression))(i)
}

pub fn parse_condition<F: TryFrom<String>>(i: &str) -> Result<Condition<F>, Error> {
    match condition(i) {
        Ok((rest, cond)) => {
            assert_eq!(rest, "");
            Ok(cond)
        }
        Err(e) => Err(Error::ParseError {
            message: e.to_string(),
        }),
    }
}
