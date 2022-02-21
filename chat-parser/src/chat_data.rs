use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, newline, not_line_ending, space0, space1};
use nom::combinator::{map, not, opt};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{pair, preceded, separated_pair, terminated, tuple};
use nom::IResult;
use std::fmt;

/// Token for containing all data included in the CHAT file
pub struct Token {
    /// Cantonese characters
    word: String,
    // /// Jyutping romanization
    jyutping: String,
    // /// Part-of-speech
    // pos: String,
}

impl Token {
    pub fn parse(raw: &str) -> IResult<&str, (Vec<&str>, Vec<(&str, &str)>)> {
        separated_pair(Self::parse_characters, tag("\n"), Self::parse_jyutping)(raw)
    }

    pub fn parse_characters(raw: &str) -> IResult<&str, Vec<&str>> {
        let match_word = separated_list0(space1, nom_unicode::complete::alpha1);

        preceded(tuple((tag("*"), alpha1, tag(":"), space0)), match_word)(raw)
    }

    pub fn parse_jyutping(raw: &str) -> IResult<&str, Vec<(&str, &str)>> {
        let match_jyutping = separated_list1(
            space1,
            separated_pair(
                nom::character::complete::alpha1,
                tag("|"),
                nom::character::complete::alphanumeric1,
            ),
        );

        preceded(pair(tag("%mor:"), space0), match_jyutping)(raw)
    }
}

#[cfg(test)]
mod test_parse_characters {
    use super::*;

    #[test]
    fn should_handle_words() {
        assert_eq!(
            Token::parse_characters("*XXB:	開 冷氣"),
            Ok(("", vec!["開", "冷氣"]))
        );
    }

    #[test]
    fn should_handle_punctuation() {
        assert_eq!(
            Token::parse_characters("*XXB:	見 到 , 呵 ?"),
            Ok(("", vec!["見", "到", "呵"]))
        );
    }
}

#[cfg(test)]
mod test_parse_jyutping {
    use super::*;

    #[test]
    fn should_handle_jyutping() {
        assert_eq!(
            Token::parse_jyutping("%mor:	a|hou2 y|aa1 ."),
            Ok(("", vec![("a", "hou2"), ("y", "aa1")]))
        );
    }
}

#[derive(Debug)]
pub enum ChatEncoding {
    Utf8,
    Unknown,
}

#[derive(Debug)]
pub struct ChatMeta {
    pub encoding: ChatEncoding,
}

#[derive(Debug)]
pub struct ChatData {
    pub meta: ChatMeta,
}

impl fmt::Display for ChatEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChatEncoding::Utf8 => write!(f, "utf8"),
            ChatEncoding::Unknown => write!(f, "unknown"),
        }
    }
}
