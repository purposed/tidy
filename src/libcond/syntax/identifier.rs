use nom::{
    character::complete::{alphanumeric1, anychar, char},
    combinator::{all_consuming, opt, recognize, verify},
    multi::many0_count,
    sequence::pair,
    sequence::preceded,
    IResult,
};

pub fn identifier(s: &str) -> IResult<&str, &str> {
    recognize(all_consuming(pair(
        verify(anychar, |&c| c.is_lowercase()),
        many0_count(preceded(opt(char('_')), alphanumeric1)),
    )))(s)
}

#[cfg(test)]
mod tests {
    use super::identifier;

    #[test]
    fn valid_identifier() {
        let (r, l) = identifier("bingbong").unwrap();
        assert_eq!(r, "");
        assert_eq!(l, "bingbong");
    }

    #[test]
    fn identifier_underscore() {
        let (r, l) = identifier("bing_bong").unwrap();
        assert_eq!(r, "");
        assert_eq!(l, "bing_bong");
    }
}
