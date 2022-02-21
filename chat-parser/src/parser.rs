use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending, newline, not_line_ending, space0};
use nom::combinator::{opt, map};
use nom::sequence::{pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;

// for ignoring all metadata
fn match_metadata(raw: &str) -> IResult<&str, (&str, Option<&str>)> {
    preceded(
        tag("@"),
        pair(
            alphanumeric1,
            opt(preceded(pair(tag(":"), space0), not_line_ending)),
        ),
    )(raw)
}

#[cfg(test)]
mod test_match_metadata {
    use super::*;

    #[test]
    fn should_match_metadata_with_values() {
        assert_eq!(
            match_metadata("@Languages: yue , eng"),
            Ok(("", ("Languages", Some("yue , eng"))))
        );
    }

    #[test]
    fn should_match_metadata_without_values() {
        assert_eq!(match_metadata("@Begin"), Ok(("", ("Begin", None))));
    }

    // #[test]
    // fn should_match_mixed_metadata() {
    // assert_eq!(
    // match_metadata("@Begin\n@Languages: yue , eng\n"),
    // Ok(("", ("Begin", None)))
    // );
    // }
}

//TODO Handle morphological
// fn match_morphological(raw: &str) -> IResult<&str, &str> {
// preceded(pair(tag("%mor:"), space0), alphanumeric1)(raw)
// }

// #[cfg(test)]
// mod test_match_morphological {
// use super::*;

// #[test]
// fn should_match_metadata_without_values() {
// assert_eq!(match_morphological("%mor: v1|jau5 ."), Ok(("", "")));
// }
// }
