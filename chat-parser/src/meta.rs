use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, not_line_ending, space0};
use nom::combinator::opt;

use nom::sequence::{pair, preceded};
use nom::IResult;

/// Match all metadata
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
}

#[derive(Debug, PartialEq)]
pub struct Meta {
    pub encoding: String,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            encoding: "unknown".to_owned(),
        }
    }
}

//REF https://talkbank.org/manuals/CHAT.pdf
impl Meta {
    pub fn parse(raw: &str) -> IResult<&str, (&str, Option<&str>)> {
        match_metadata(raw)
    }
}

// fold_many1(
// separated_list1(line_ending, match_metadata),
// Meta::default,
// |mut acc: Meta, results| -> Self {
// //TODO Apply the value for meta with values
// for (kind, _value) in results.into_iter() {
// match kind {
// "UTF8" => {
// acc.encoding = kind.to_owned();
// }
// _ => (),
// }
// }

// acc
// },
// )(raw)
