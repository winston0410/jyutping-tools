use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending, newline, not_line_ending, space0};
use nom::combinator::{map, opt};
use nom::multi::{fold_many0, separated_list0};
use nom::sequence::{pair, preceded, terminated, tuple};
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
}

// #[derive(Debug)]
// pub enum ChatEncoding {
// Utf8,
// Unknown,
// }

//TODO Handle metadata properly later
// impl fmt::Display for ChatEncoding {
// fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// match self {
// ChatEncoding::Utf8 => write!(f, "utf8"),
// ChatEncoding::Unknown => write!(f, "unknown"),
// }
// }
// }

#[derive(Debug, PartialEq)]
pub struct Meta {
    encoding: String,
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
    pub fn parse(raw: &str) -> IResult<&str, Meta> {
        fold_many0(
            separated_list0(line_ending, match_metadata),
            Meta::default,
            |acc: Meta, results| 
            // mutate the Meta
            acc,
        )(raw)
    }
}

// for (kind, value) in results.into_iter() {
// match kind {
// "UTF8" => {
// acc.encoding = kind.to_owned();
// }
// _ => (),
// }
// }

#[cfg(test)]
mod test_match_multiple_metadata {
    use super::*;

    #[test]
    fn should_parse_meta() {
        assert_eq!(
            Meta::parse(
                "@UTF8
@Begin
@End"
            ),
            Ok((
                "",
                Meta {
                    encoding: "UTF8".to_owned()
                }
            ))
        );
    }
}
