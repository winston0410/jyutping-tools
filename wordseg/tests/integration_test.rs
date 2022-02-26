#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn should_panic_when_given_invalid_max_word_length() {
        wordseg::Segmenter::default().max_word_length(0);
    }

    #[test]
    fn should_predicts_english() {
        let mut segmenter = wordseg::Segmenter::default().max_word_length(4);

        let data: Vec<String> = vec![
            "this", "is", "a", "sentence", "that", "is", "not", "a", "sentence",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        segmenter.fit(&data);

        let result = segmenter.predict("thatisadog");

        assert_eq!(result, vec!["that", "is", "a", "d", "o", "g"]);
    }

    #[test]
    fn should_predicts_unicode_characters() {
        let mut segmenter = wordseg::Segmenter::default().max_word_length(4);

        let data: Vec<String> = vec!["我", "係", "香港", "人"]
            .into_iter()
            .map(String::from)
            .collect();

        segmenter.fit(&data);

        let result = segmenter.predict("佢哋係香港人");

        assert_eq!(result, vec!["佢", "哋", "係", "香港", "人"]);
    }
}
