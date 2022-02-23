use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, line_ending, newline, not_line_ending, space0};
use nom::combinator::{map, opt};
use nom::multi::separated_list0;
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::IResult;

// for ignoring all metadata
fn match_metadata(raw: &str) -> IResult<&str, Meta> {
    map(
        preceded(
            tag("@"),
            pair(
                alphanumeric1,
                opt(preceded(pair(tag(":"), space0), not_line_ending)),
            ),
        ),
        |(kind, value): (&str, Option<&str>)| Meta {
            kind: kind.to_owned(),
            value: value.map(String::from),
        },
    )(raw)
}

#[cfg(test)]
mod test_match_metadata {
    use super::*;

    #[test]
    fn should_match_metadata_with_values() {
        assert_eq!(
            match_metadata("@Languages: yue , eng"),
            Ok((
                "",
                Meta {
                    kind: "Languages".to_owned(),
                    value: Some("yue , eng".to_owned())
                }
            ))
        );
    }

    #[test]
    fn should_match_metadata_without_values() {
        assert_eq!(
            match_metadata("@Begin"),
            Ok((
                "",
                Meta {
                    kind: "Begin".to_owned(),
                    value: None
                }
            ))
        );
    }
}

// #[derive(Debug)]
// pub enum ChatEncoding {
// Utf8,
// Unknown,
// }

#[derive(Debug, PartialEq)]
pub struct Meta {
    kind: String,
    value: Option<String>,
}

impl Meta {
    pub fn parse(raw: &str) -> IResult<&str, Vec<Meta>> {
        separated_list0(tag("\n"), match_metadata)(raw)
    }
}

#[cfg(test)]
mod test_match_multiple_metadata {
    use super::*;

    #[test]
    fn should_match_multiple_metadata() {
        assert_eq!(
            Meta::parse(
                "@Begin
@End"
            ),
            Ok((
                "",
                vec![
                    Meta {
                        kind: "Begin".to_owned(),
                        value: None
                    },
                    Meta {
                        kind: "End".to_owned(),
                        value: None
                    }
                ]
            ))
        );
    }
}

//TODO Handle metadata properly later
// impl fmt::Display for ChatEncoding {
// fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// match self {
// ChatEncoding::Utf8 => write!(f, "utf8"),
// ChatEncoding::Unknown => write!(f, "unknown"),
// }
// }
// }
