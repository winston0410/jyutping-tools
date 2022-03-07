pub fn structure_result((word, jyutpings): (&str, Vec<&str>)) -> (String, Vec<String>) {
    (
        word.to_owned(),
        jyutpings.into_iter().map(String::from).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::structure_result;
    use rscantonese::data::wordshk;
    use rscantonese::RsCantonese;

    #[test]
    fn should_convert_cantonese_to_jyutping() {
        let mut rscantonese = RsCantonese::default();

        rscantonese.train(&wordshk());

        let result = rscantonese.characters_to_jyutping("香港人講廣東話");

        let expected_result: Vec<(String, Vec<String>)> = vec![
            ("香港人", vec!["hoeng1gong2jan4"]),
            ("講", vec!["gong2"]),
            ("廣東話", vec!["gwong2dung1waa2"]),
        ]
        .into_iter()
        .map(structure_result)
        .collect();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_handle_unknown_input() {
        let mut rscantonese = RsCantonese::default();

        rscantonese.train(&wordshk());

        let result = rscantonese.characters_to_jyutping("香港사람");

        let expected_result: Vec<(String, Vec<String>)> = vec![
            ("香港", vec!["hoeng1gong2"]),
            ("사", vec!["unknown"]),
            ("람", vec!["unknown"]),
        ]
        .into_iter()
        .map(structure_result)
        .collect();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_handle_punctuation() {
        let mut rscantonese = RsCantonese::default();

        rscantonese.train(&wordshk());

        let result = rscantonese
            .characters_to_jyutping("，！？；：（ ）［］【 】。『 』「 」、·《》〈〉…～—");

        let expected_result: Vec<(String, Vec<String>)> = vec![
            ("，", vec!["，"]),
            ("？", vec!["？"]),
            ("。", vec!["。"]),
            (",", vec![","]),
            ("?", vec!["?"]),
            (".", vec!["."]),
        ]
        .into_iter()
        .map(structure_result)
        .collect();

        assert_eq!(result, expected_result);
    }
}
