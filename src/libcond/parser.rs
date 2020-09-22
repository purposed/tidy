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

use crate::condition::{BaseCondition, BoolOperator, Condition, FieldOperator};

#[derive(Clone, Debug, Snafu)]
pub enum Error {
    ParseError { message: String },
}

fn whitespace(i: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn field_operator(i: &str) -> IResult<&str, FieldOperator> {
    map(
        delimited(
            whitespace,
            alt((
                tag("=="),
                tag("!="),
                tag("<"),
                tag(">"),
                tag("<="),
                tag(">="),
            )),
            whitespace,
        ),
        |c| FieldOperator::try_from(c).unwrap(),
    )(i)
}

fn bool_operator(i: &str) -> IResult<&str, BoolOperator> {
    map(
        delimited(
            whitespace,
            alt((tag("&&"), tag("||"), tag("-|"))),
            whitespace,
        ),
        |c| BoolOperator::try_from(c).unwrap(),
    )(i)
}

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

pub fn parse_condition<F: TryFrom<String>>(i: &str) -> Result<Condition<F>, Error> {
    match expression(i) {
        Ok((rest, cond)) => {
            assert_eq!(rest, "");
            Ok(cond)
        }
        Err(e) => Err(Error::ParseError {
            message: e.to_string(),
        }),
    }
}
