pub use chat_parser::corpus::Corpus as InputToken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
// Use feature = "tests" instead of test, as simply using test doesnt cover integrated_test
#[cfg_attr(feature = "tests", derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WordToken {
    pub pos: String,
    pub jyutping: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "tests", derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PuncutationToken(pub String);

/// This is the enum used for training RsCantonese
#[derive(Debug, Clone)]
#[cfg_attr(feature = "tests", derive(PartialEq))]
pub enum OutputToken {
    Puncutation(PuncutationToken),
    Word(Vec<WordToken>),
}
