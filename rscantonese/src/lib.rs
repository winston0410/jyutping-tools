use chat_parser::corpus::Corpus;
use std::collections::HashMap;
use wordseg::Segmenter;
pub mod data;
pub mod token;
use token::{InputToken, OutputToken};
pub mod transformer;

#[derive(Default)]
pub struct RsCantonese {
    pub segmenter: Segmenter,
    pub conversion_dict: HashMap<String, Vec<token::OutputToken>>,
}

/// Output type for using RsCantonese to parse unsegmented string
pub type Result = Vec<(String, Option<Vec<OutputToken>>)>;

impl RsCantonese {
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
        let segmented = self.segmenter.predict(unsegmented);

        // Need to handle punctuation here, as their length is one only
        segmented
            .into_iter()
            .map(|word| {
                let jyutpings = self.conversion_dict.get(&word).map(ToOwned::to_owned);
                (word, jyutpings)
            })
            // .map(transformer::handle_punctuations)
            .collect()
    }

    /// Train the model
    pub fn train(&mut self, data: &[Corpus]) -> &Self {
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
                    cur.push(OutputToken {
                        jyutping: jyutping.to_owned(),
                        pos: pos.to_owned(),
                    })
                })
                .or_insert(vec![OutputToken {
                    jyutping: jyutping.to_owned(),
                    pos: pos.to_owned(),
                }]);
        }

        self.segmenter
            .fit(self.conversion_dict.keys())
            .update_constraint();
        self
    }
}

#[cfg(test)]
mod unit_tests {
    use super::token::{InputToken, OutputToken};
    use super::RsCantonese;

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

        assert_eq!(
            tokens,
            &vec![
                OutputToken {
                    jyutping: "coeng4".to_owned(),
                    pos: "a".to_owned(),
                },
                OutputToken {
                    jyutping: "zoeng2".to_owned(),
                    pos: "v".to_owned(),
                }
            ]
        );
    }
}
