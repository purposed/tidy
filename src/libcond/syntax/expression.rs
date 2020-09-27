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
        /*
        |(field_name, field_op, value)| {
            Condition::FieldCondition(BaseCondition::new(value, field_op, field_name))
        },
        */
        |(field_name, field_op, value)| {
            Condition::FieldCondition(BaseCondition::new(value, field_op, field_name))
        },
    )(i)
}

#[cfg(test)]
mod tests {

    use std::convert::TryFrom;

    use super::expression;
    use crate::condition::{BaseCondition, Condition, FieldOperator};

    #[derive(Clone, Debug, PartialEq)]
    enum TestField {
        Hello,
        World,
    }

    struct Error {}

    impl TryFrom<String> for TestField {
        type Error = Error;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            match value.as_ref() {
                "hello" => Ok(TestField::Hello),
                "w_o_rld" => Ok(TestField::World),
                _ => Err(Error {}),
            }
        }
    }

    #[test]
    fn field_equal_string() {
        let (r, l): (&str, Condition<TestField>) = expression("@hello==\"bing\"").unwrap();
        assert_eq!(r, "");

        let expected_condition = Condition::FieldCondition(BaseCondition::new(
            String::from("bing"),
            FieldOperator::Equal,
            TestField::Hello,
        ));
        assert_eq!(l, expected_condition);
    }
}
