use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InputToken {
    /// Cantonese characters
    pub word: String,
    // /// Jyutping romanization
    pub jyutping: String,
    // /// Part-of-speech
    pub pos: String,
}

#[derive(Debug, Clone)]
// Use feature = "tests" instead of test, as simply using test doesnt cover integrated_test
#[cfg_attr(feature = "tests", derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WordToken {
    pub pos: String,
    pub jyutping: String,
    //TODO Change the type of jyutping into Vec and handle multiple pronunciations per token
    // pub jyutping: Vec<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "tests", derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PuncutationToken(pub String);

/// This is the enum used for training RsCantonese
#[derive(Debug, Clone)]
#[cfg_attr(feature = "tests", derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(untagged)]
pub enum OutputToken {
    Puncutation(PuncutationToken),
    Word(Vec<WordToken>),
}
