//REF https://www.compart.com/en/unicode/category/Po
//REF https://www.kerryr.net/pioneers/ascii3.htm
//NOTE doesnt handle cantonese punctuation right now
pub fn is_punctuation(char: char) -> bool {
    let bytes = char as u32;
    // // 33 - 47
    (bytes >= 0x0021 && bytes <= 0x002F) || 
    // 58 - 64
    (bytes >= 0x003A && bytes <= 0x0040) ||
    // 91 - 96
    (bytes >= 0x005B && bytes <= 0x0060) ||
    // 123 - 126
    (bytes >= 0x007B && bytes <= 0x007E)
}

#[cfg(test)]
mod test_is_punctuation {
    use super::*;

    #[test]
    fn should_match_punctuation() {
        assert_eq!(is_punctuation(','), true);
    }

    #[test]
    fn should_not_match_space() {
        assert_eq!(is_punctuation(' '), false);
    }

    #[test]
    fn should_not_match_number() {
        assert_eq!(is_punctuation('2'), false);
    }

    #[test]
    fn should_not_match_characters() {
        assert_eq!(is_punctuation('a'), false);
    }
}
