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
                tag("<="),
                tag(">="),
                tag("<"),
                tag(">"),
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

    macro_rules! field_op {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    let (r, op) = field_operator(input).unwrap();
                    assert_eq!(r, "");
                    assert_eq!(op, expected);
                }
            )*
        }
    }

    macro_rules! bool_op {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    let (r, op) = bool_operator(input).unwrap();
                    assert_eq!(r, "");
                    assert_eq!(op, expected);
                }
            )*
        }
    }

    field_op! {
        field_operator_eq: ("==\t", FieldOperator::Equal),
        field_operator_neq: (" != ", FieldOperator::NotEqual),
        field_operator_gt: (" >", FieldOperator::Gt),
        field_operator_lt: (" < ", FieldOperator::Lt),
        field_operator_leq: (" <= ", FieldOperator::Leq),
        field_operator_geq: ("\t>= ", FieldOperator::Geq),
    }

    bool_op! {
        bool_operator_or: ("||\n", BoolOperator::Or),
        bool_operator_and: ("\r&&\n", BoolOperator::And),
        bool_operator_xor: ("\t\n-|\n", BoolOperator::Xor),
    }
}
