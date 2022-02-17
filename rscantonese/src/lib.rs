use chat_parser::Parser;
use wordseg::Segmenter;

/// Convert cantonese characters to jyutping
pub fn characters_to_jyutping(unsegmented: &str) -> Vec<(String, String)> {
    let segment = Segmenter::default();
    return vec![("hello".to_owned(), "world".to_owned())];
}
