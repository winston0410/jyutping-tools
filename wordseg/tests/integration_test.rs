#[cfg(test)]
mod forward_maximal {
    #[test]
    fn should_predicts_english() {
        let data: Vec<String> = vec![
            "this", "is", "a", "sentence", "that", "is", "not", "a", "sentence",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let mut segmenter = wordseg::Segmenter::default();

        // the calling of these functions should not affect the ownership of segmenter
        segmenter.fit(&data).update_constraint();

        let result = segmenter.forward_predict("thatisadog");

        assert_eq!(result, vec!["that", "is", "a", "d", "o", "g"]);
    }

    #[test]
    // This test is to show that incorrect result you could get by simple using forward maximal
    // segmentation
    fn should_handle_unicode() {
        let data: Vec<String> = vec!["研究", "研究生", "生命", "命", "的", "起源"]
            .into_iter()
            .map(String::from)
            .collect();

        let mut segmenter = wordseg::Segmenter::default();

        segmenter.fit(&data).update_constraint();

        let result = segmenter.forward_predict("研究生命的起源");

        assert_eq!(result, vec!["研究生", "命", "的", "起源"]);
    }

    #[test]
    fn should_handle_overlapping_words() {
        let data: Vec<String> = vec!["香港", "人", "香港人", "講", "廣東", "廣東話"]
            .into_iter()
            .map(String::from)
            .collect();

        let mut segmenter = wordseg::Segmenter::default();

        segmenter.fit(&data).update_constraint();

        let result = segmenter.forward_predict("香港人講廣東話");

        assert_eq!(result, vec!["香港人", "講", "廣東話"]);
    }
}

#[cfg(test)]
mod backward_maximal {
    #[test]
    // This test is to show that you could get better result by simple using reverse maximal
    // segmentation
    fn should_handle_unicode() {
        let data: Vec<String> = vec!["研究", "研究生", "生命", "命", "的", "起源"]
            .into_iter()
            .map(String::from)
            .collect();

        let mut segmenter = wordseg::Segmenter::default();

        segmenter.fit(&data).update_constraint();

        let result = segmenter.reverse_predict("研究生命的起源");

        assert_eq!(result, vec!["研究", "生命", "的", "起源"]);
    }
}

#[cfg(test)]
mod bidirectional_maximal {
    #[test]
    fn should_handle_unicode() {
        let data: Vec<String> = vec!["研究", "研究生", "生命", "命", "的", "起源"]
            .into_iter()
            .map(String::from)
            .collect();

        let mut segmenter = wordseg::Segmenter::default();

        segmenter.fit(&data).update_constraint();

        let result = segmenter.bidirectional_predict("研究生命的起源");

        assert_eq!(result, vec!["研究", "生命", "的", "起源"]);
    }
}
