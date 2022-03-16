pub use chat_parser::corpus::Corpus as InputToken;
use serde::{Deserialize, Serialize};

/// This is the struct used for training RsCantonese
#[derive(Debug, Clone)]
// Use feature = "tests" instead of test, as simply using test doesnt cover integrated_test
#[cfg_attr(feature = "tests", derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OutputToken {
    pub pos: String,
    pub jyutping: String,
}
