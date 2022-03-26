use chat_parser::corpus::Corpus;
use std::collections::HashMap;
use wordseg::Segmenter;
pub mod data;
pub mod token;
use token::{InputToken, OutputToken, PuncutationToken, WordToken};
pub mod transformer;

#[derive(Default)]
pub struct RsCantonese {
    pub segmenter: Segmenter,
    pub conversion_dict: HashMap<String, token::OutputToken>,
}

/// Output type for using RsCantonese to parse unsegmented string
pub type Result = Vec<(String, Option<OutputToken>)>;

impl RsCantonese {
    ///Helper function to get the first and longest unknown token
    pub fn find_unknown(result: &[(String, Option<OutputToken>)]) -> String {
        let mut unknown = "".to_owned();
        let mut found = false;

        for (key, token) in result.iter() {
            if token.is_none() {
                if !found {
                    found = true;
                }
                unknown.push_str(key);
            } else {
                //This will execute after the iterated on the first known token
                if found {
                    return unknown;
                }
            }
        }

        unknown
    }

    /// Helper function for check if the result has unknown value
    pub fn has_unknown(result: &[(String, Option<OutputToken>)]) -> bool {
        result.iter().any(|(_, token)| token.is_none())
    }

    // Reason why we use Into and AsRef
    //REF https://discord.com/channels/442252698964721669/947648480799752192

    /// Parse input and generate result with Jyutping and Part-of-Speech. Words that cannot be converted into Jyutping will be represented with None
    ///
    /// This function is the combination for characters_to_jyutping, segment, and pos in
    /// PyCantonese
    pub fn parse<T>(&self, unsegmented: T) -> Result
    where
        T: AsRef<str> + Into<String>,
    {
        let segmented = self.segmenter.bidirectional_predict(unsegmented);

        // Need to handle punctuation here, as their length is one only
        segmented
            .into_iter()
            .map(|word| {
                let output_token = self.conversion_dict.get(&word).map(ToOwned::to_owned);
                (word, output_token)
            })
            .map(transformer::handle_punctuations)
            .collect()
    }

    /// Train the model with multiple tokens
    pub fn train(&mut self, data: &[InputToken]) -> &Self {
        for InputToken {
            jyutping,
            pos,
            word,
        } in data
        {
            self.conversion_dict
                .entry(word.to_owned())
                .and_modify(|cur| {
                    //Handle merging logic here
                    match cur {
                        OutputToken::Word(tokens) => {
                            tokens.push(WordToken {
                                jyutping: jyutping.to_owned(),
                                pos: pos.to_owned(),
                            });
                        }
                        OutputToken::Puncutation(..) => {
                            unreachable!()
                        }
                    }
                })
                .or_insert(OutputToken::Word(vec![WordToken {
                    jyutping: jyutping.to_owned(),
                    pos: pos.to_owned(),
                }]));
        }

        self.segmenter
            .fit(self.conversion_dict.keys())
            .update_constraint();
        self
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn should_store_multiple_tokens_per_key() {
        let mut rscantonese = RsCantonese::default();

        rscantonese.train(&vec![
            InputToken {
                word: "長".to_owned(),
                jyutping: "coeng4".to_owned(),
                pos: "a".to_owned(),
            },
            InputToken {
                word: "長".to_owned(),
                jyutping: "zoeng2".to_owned(),
                pos: "v".to_owned(),
            },
        ]);

        let tokens = rscantonese.conversion_dict.get("長").unwrap();

        //NOTE False alarm as rust-analyzer doesnt not have `tests` feature on when running the test
        assert_eq!(
            tokens,
            &OutputToken::Word(vec![
                WordToken {
                    jyutping: "coeng4".to_owned(),
                    pos: "a".to_owned(),
                },
                WordToken {
                    jyutping: "zoeng2".to_owned(),
                    pos: "v".to_owned(),
                }
            ]),
        );
    }

    #[test]
    fn should_detect_unknown() {
        let mut rscantonese = RsCantonese::default();

        let tokens = rscantonese.parse("長");

        let result = RsCantonese::has_unknown(&tokens);

        assert_eq!(result, true);
    }

    #[test]
    fn should_find_unknown() {
        let mut rscantonese = RsCantonese::default();

        rscantonese.train(&vec![InputToken {
            word: "香港".to_owned(),
            jyutping: "hoeng1gong2".to_owned(),
            pos: "n".to_owned(),
        }]);

        let tokens = rscantonese.parse("我係香港人");

        let unknown = RsCantonese::find_unknown(&tokens);

        assert_eq!(unknown, "我係".to_owned());
    }
}
