use crate::predicates::is_punctuation;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, line_ending, space0, space1};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{pair, preceded, separated_pair, tuple};
use nom::IResult;

/// Token for containing all data included in the CHAT file
#[derive(Debug, PartialEq)]
pub struct Corpus {
    /// Cantonese characters
    pub word: String,
    // /// Jyutping romanization
    pub jyutping: String,
    // /// Part-of-speech
    pub pos: String,
}

impl Corpus {
    pub fn parse(raw: &str) -> IResult<&str, Vec<Self>> {
        let corpus_parser =
            separated_pair(Self::parse_characters, line_ending, Self::parse_jyutping);

        map(corpus_parser, |(words, morphology)| -> Vec<Self> {
            let mut result: Vec<Self> = Vec::new();
            let iter = words.iter().zip(morphology.iter());

            for (word, (pos, jyutping)) in iter {
                if *jyutping != "" {
                    result.push(Self {
                        jyutping: (*jyutping).to_owned(),
                        pos: (*pos).to_owned(),
                        word: (*word).to_owned(),
                    })
                }
            }

            result
        })(raw)
    }

    /// Function for parsing characters. Punctuation will not be filtered out here for performance
    /// concern
    pub fn parse_characters(raw: &str) -> IResult<&str, Vec<&str>> {
        let match_word = separated_list0(
            space1,
            alt((nom_unicode::complete::alpha1, take_while(is_punctuation))),
        );

        preceded(tuple((tag("*"), alpha1, tag(":"), space0)), match_word)(raw)
    }

    /// Function for parsing jyutping. Punctuation will not be filtered out here for performance
    /// concern
    pub fn parse_jyutping(raw: &str) -> IResult<&str, Vec<(&str, &str)>> {
        let match_jyutping = separated_list0(
            space1,
            alt((
                separated_pair(
                    nom::character::complete::alpha1,
                    tag("|"),
                    nom::character::complete::alphanumeric1,
                ),
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
            Corpus::parse_characters("*XXB:	開 冷氣"),
            Ok(("", vec!["開", "冷氣"]))
        );
    }

    #[test]
    fn should_handle_punctuation() {
        assert_eq!(
            Corpus::parse_characters("*XXB:	見 到 , 呵 ?"),
            Ok(("", vec!["見", "到", ",", "呵", "?"]))
        );
    }
}

#[cfg(test)]
mod test_parse_jyutping {
    use super::*;

    #[test]
    fn should_handle_jyutping() {
        assert_eq!(
            Corpus::parse_jyutping("%mor:	a|hou2 y|aa1 ."),
            Ok(("", vec![("a", "hou2"), ("y", "aa1"), (".", "")]))
        );
    }
}

#[cfg(test)]
mod test_parse {
    use super::*;

    #[test]
    fn should_parse_as_token() {
        assert_eq!(
            Corpus::parse(
                "*XXA:	好 吖 .
%mor:	a|hou2 y|aa1 ."
            ),
            Ok((
                "",
                vec![
                    Corpus {
                        word: "好".to_owned(),
                        jyutping: "hou2".to_owned(),
                        pos: "a".to_owned()
                    },
                    Corpus {
                        word: "吖".to_owned(),
                        jyutping: "aa1".to_owned(),
                        pos: "y".to_owned()
                    }
                ]
            ))
        );
    }
}
