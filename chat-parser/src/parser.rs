use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, newline};
use nom::sequence::{pair, preceded, terminated};
use nom::IResult;

///matching the encoding of the file
fn match_encoding(raw: &str) -> IResult<&str, &str> {
    terminated(
        preceded(tag("@"), alphanumeric1),
        pair(newline, tag("@Begin")),
    )(raw)
}

#[cfg(test)]
mod match_encoding {
    use super::*;

    #[test]
    fn test_match_encoding() {
        assert_eq!(
            match_encoding(
                "@UTF8
@Begin"
            ),
            Ok(("", "UTF8"))
        );
    }
}
