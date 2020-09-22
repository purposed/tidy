use std::convert::TryFrom;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::anychar,
    combinator::map,
    multi::many0,
    sequence::delimited,
    IResult,
};

use crate::condition::{BoolOperator, FieldOperator};

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
