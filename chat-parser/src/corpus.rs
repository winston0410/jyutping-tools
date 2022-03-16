use crate::predicates::{is_morphology, is_punctuation};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::{alpha1, line_ending, space0, space1};
use nom::combinator::map;
use nom::multi::{many0, separated_list0};
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
        let corpus_parser = many0(alt((Self::parse_characters, Self::parse_jyutping)));

        // map(corpus_parser, |(words, morphology)| -> Vec<Self> {
        map(corpus_parser, |word| -> Vec<Self> {
            // let mut result: Vec<Self> = Vec::new();
            // let iter = words.iter().zip(morphology.iter());

            // for (word, (pos, jyutping)) in iter {
            // if *jyutping != "" {
            // result.push(Self {
            // jyutping: (*jyutping).to_owned(),
            // pos: (*pos).to_owned(),
            // word: (*word).to_owned(),
            // })
            // }
            // }

            // result
            println!("{:?}", word);

            vec![Corpus {
                word: "".to_owned(),
                pos: "".to_owned(),
                jyutping: "".to_owned(),
            }]
        })(raw)
    }

    /// Function for parsing characters. Punctuation will not be filtered out here for performance
    /// concern
    pub fn parse_characters(raw: &str) -> IResult<&str, Vec<&str>> {
        let match_word = separated_list0(
            alt((space1, line_ending)),
            alt((nom_unicode::complete::alpha1, take_while(is_punctuation))),
        );

        preceded(
            tuple((line_ending, tag("*"), alpha1, tag(":"), space0)),
            match_word,
        )(raw)
    }

    /// Function for parsing jyutping. Punctuation will not be filtered out here for performance
    /// concern.
    ///
    /// This function will precede linebreak and %mor, as it is the unique divider
    pub fn parse_jyutping(raw: &str) -> IResult<&str, Vec<&str>> {
        let match_jyutping = separated_list0(alt((space1, line_ending)), take_while(is_morphology));

        preceded(tuple((line_ending, tag("%mor:"), space0)), match_jyutping)(raw)
    }
}

#[cfg(test)]
mod test_parse_characters {
    use super::*;

    #[test]
    fn should_handle_words() {
        assert_eq!(
            Corpus::parse_characters("\n*XXB:	開 冷氣"),
            Ok(("", vec!["開", "冷氣"]))
        );
    }

    #[test]
    fn should_handle_punctuation() {
        assert_eq!(
            Corpus::parse_characters("\n*XXB:	見 到 , 呵 ?"),
            Ok(("", vec!["見", "到", ",", "呵", "?"]))
        );
    }

    #[test]
    fn should_handle_early_link_break() {
        assert_eq!(
            Corpus::parse_characters("\n*XXB:	見 到 ,\n 呵 ?"),
            Ok((
                "",
                vec![
                    "見", "到", ",", //FIXME Remove Incorrectly collected token
                    "", "呵", "?"
                ]
            ))
        );
    }
}

#[cfg(test)]
mod test_parse_jyutping {
    use super::*;

    #[test]
    fn should_handle_jyutping() {
        assert_eq!(
            Corpus::parse_jyutping("\n%mor:	a|hou2 y|aa1 ."),
            Ok(("", vec!["a|hou2", "y|aa1", "."]))
        );
    }

    // #[test]
    // fn should_handle_early_line_break() {
    // assert_eq!(
    // Corpus::parse_jyutping(
    // "\n%mor:	c|gam2 v|hai6 y|laa1 , v|gong2 u|faan1 r|ngo5dei6 n|gan6fong3
    // y|aa1 ."
    // ),
    // Ok((
    // "",
    // vec![
    // ("c", "gam2"),
    // ("v", "hai6"),
    // ("y", "laa1"),
    // (",", ""),
    // ("v", "gong2"),
    // ("u", "faan1"),
    // ("r", "ngo5dei6"),
    // ("n", "gan6fong3"),
    // //FIXME Remove Incorrectly collected token
    // ("", ""),
    // ("y", "aa1"),
    // (".", ""),
    // ]
    // ))
    // );
    // }
}

#[cfg(test)]
mod test_parse {
    use super::*;

    #[test]
    fn should_parse_as_token() {
        assert_eq!(
            // Corpus::parse("\n*XXA:	好 吖 . \n%mor:	a|hou2 y|aa1 ."),
            Corpus::parse("\n*XXA:	好 吖\n%mor:	a|hou2 y|aa1"),
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
