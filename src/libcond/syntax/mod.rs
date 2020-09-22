mod operator;

use nom::{bytes::complete::take_while, IResult};

use snafu::Snafu;

#[derive(Clone, Debug, Snafu)]
pub enum Error {
    ParseError { message: String },
}

fn whitespace(i: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}
