use nom::{
    bytes::complete::{escaped_transform, tag},
    character::complete::{digit1, none_of},
    combinator::{map, map_parser, recognize},
    multi::{many0, separated_list},
    sequence::delimited,
    IResult,
};

pub fn string_literal(input: &str) -> IResult<&str, String> {
    let seq = recognize(separated_list(tag("\"\""), many0(none_of("\""))));
    let unquote = escaped_transform(none_of("\""), '"', tag("\""));
    let res = delimited(tag("\""), map_parser(seq, unquote), tag("\""))(input)?;
    Ok(res)
}

pub fn number(i: &str) -> IResult<&str, String> {
    map(digit1, String::from)(i)
}

#[cfg(test)]
mod tests {

    use super::{number, string_literal};

    #[test]
    fn literal_correct_form() {
        let (r, l) = string_literal("\"bing\"").unwrap();
        assert_eq!(r, "");
        assert_eq!(&l, "bing");
    }

    #[test]
    fn empty_literal_invalid() {
        assert!(string_literal("\"\"").is_err());
    }

    #[test]
    fn parse_number() {
        let (r, l) = number("42").unwrap();
        assert_eq!(r, "");
        assert_eq!(&l, "42");
    }
}
