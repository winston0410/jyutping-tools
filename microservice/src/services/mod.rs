pub type CharsToJyutpingResult = Vec<(String, String)>;

pub fn chars_to_jyutping(text: String) -> CharsToJyutpingResult {
    vec![("chinese".to_owned(), "jyutping".to_owned())]
}

pub type SegmentResult = Vec<String>;

pub fn segment_chars(text: String) -> SegmentResult {
    vec!["chinese".to_owned()]
}
