use std::convert::TryFrom;

use nom::{bytes::complete::tag, combinator::map_res, sequence::preceded, IResult};

use super::identifier::identifier;

pub fn field<F: TryFrom<String>>(i: &str) -> IResult<&str, F> {
    map_res(preceded(tag("@"), identifier), |c| {
        F::try_from(String::from(c))
    })(i)
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use nom::IResult;

    use super::field;

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
    fn parse_valid_field() {
        let (r, l): (&str, TestField) = field("@hello").unwrap();
        assert_eq!(r, "");
        assert_eq!(l, TestField::Hello);
    }

    #[test]
    fn parse_field_underscore() {
        let (r, l): (&str, TestField) = field("@w_o_rld").unwrap();
        assert_eq!(r, "");
        assert_eq!(l, TestField::World);
    }

    #[test]
    fn parse_field_invalid() {
        let r: IResult<&str, TestField> = field("@other");
        assert!(r.is_err());
    }
}
