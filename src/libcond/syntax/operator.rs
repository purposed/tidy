use std::convert::TryFrom;

use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::delimited, IResult};

use super::whitespace;
use crate::condition::{BoolOperator, FieldOperator};

pub fn field_operator(i: &str) -> IResult<&str, FieldOperator> {
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

pub fn bool_operator(i: &str) -> IResult<&str, BoolOperator> {
    map(
        delimited(
            whitespace,
            alt((tag("&&"), tag("||"), tag("-|"))),
            whitespace,
        ),
        |c| BoolOperator::try_from(c).unwrap(),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::{bool_operator, field_operator, BoolOperator, FieldOperator};

    #[test]
    fn field_operator_eq() {
        let (r, op) = field_operator(" ==\t").unwrap();
        assert_eq!(r, "");
        assert_eq!(op, FieldOperator::Equal);
    }

    #[test]
    fn field_operator_neq() {
        let (r, op) = field_operator(" !=\t").unwrap();
        assert_eq!(r, "");
        assert_eq!(op, FieldOperator::NotEqual);
    }
}
