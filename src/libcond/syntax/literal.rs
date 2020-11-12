use nom::{
    bytes::complete::{escaped_transform, tag},
    character::complete::{digit1, none_of},
    combinator::{map, map_parser, recognize},
    multi::{many0, separated_list1},
    sequence::delimited,
    IResult,
};

use super::whitespace;

pub fn string_literal(input: &str) -> IResult<&str, String> {
    let seq = recognize(separated_list1(tag("\"\""), many0(none_of("\""))));
    let unquote = escaped_transform(none_of("\""), '"', tag("\""));
    let res = delimited(
        whitespace,
        delimited(tag("\""), map_parser(seq, unquote), tag("\"")),
        whitespace,
    )(input)?;
    Ok(res)
}

pub fn number(i: &str) -> IResult<&str, String> {
    map(delimited(whitespace, digit1, whitespace), String::from)(i)
}

#[cfg(test)]
mod tests {

    use super::{number, string_literal};

    #[test]
    fn literal_correct_form() {
        let (r, l) = string_literal(" \"bing\" ").unwrap();
        assert_eq!(r, "");
        assert_eq!(&l, "bing");
    }

    #[test]
    fn empty_literal_valid() {
        let (r, l) = string_literal("\"\"").unwrap();
        assert_eq!(r, "");
        assert_eq!(&l, "");
    }

    #[test]
    fn parse_number() {
        let (r, l) = number("42").unwrap();
        assert_eq!(r, "");
        assert_eq!(&l, "42");
    }
}
