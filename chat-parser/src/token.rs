use crate::predicates::is_punctuation;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, line_ending, newline, not_line_ending, space0, space1};
use nom::combinator::iterator;
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
        let match_word = separated_list0(
            space1,
            alt((nom_unicode::complete::alpha1, take_while(is_punctuation))),
        );

        preceded(tuple((tag("*"), alpha1, tag(":"), space0)), match_word)(raw)
    }

    pub fn parse_jyutping(raw: &str) -> IResult<&str, Vec<(&str, &str)>> {
        let match_jyutping = separated_list0(
            space1,
            alt((
                separated_pair(
                    nom::character::complete::alpha1,
                    tag("|"),
                    nom::character::complete::alphanumeric1,
                ),
                //NOTE not sure if using take_while for the second input is the correct answer for matching returning type for alt
                pair(take_while(is_punctuation), take_while(is_punctuation)),
            )),
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
