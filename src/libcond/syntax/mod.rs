mod condition;
mod expression;
mod field;
mod identifier;
mod literal;
mod operator;

use std::convert::TryFrom;

use condition::condition;

use nom::{bytes::complete::take_while, IResult};

use snafu::Snafu;

use crate::condition::Condition;

#[derive(Clone, Debug, Snafu)]
pub enum Error {
    ParseError { message: String },
}

fn whitespace(i: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
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
