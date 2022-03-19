#[cfg(test)]
mod integrated_tests {
    use rscantonese::token::{InputToken, OutputToken, PuncutationToken, WordToken};
    use rscantonese::{Result, RsCantonese};
    // use rscantonese::data::wordshk;

    // #[test]
    // fn should_convert_cantonese_to_jyutping() {
        // let mut rscantonese = RsCantonese::default();

        // rscantonese.train(&vec![
            // InputToken {
                // word: "香港人".to_owned(),
                // jyutping: "hoeng1gong2jan4".to_owned(),
                // pos: "n".to_owned(),
            // },
            // InputToken {
                // word: "講".to_owned(),
                // jyutping: "gong2".to_owned(),
                // pos: "v".to_owned(),
            // },
            // InputToken {
                // word: "廣東話".to_owned(),
                // jyutping: "gwong2dung1waa2".to_owned(),
                // pos: "n".to_owned(),
            // },
        // ]);

        // let result = rscantonese.parse("香港人講廣東話");

        // // wordshk();

        // let expected_result: Result = vec![
            // (
                // "香港人".to_owned(),
                // Some(OutputToken::Word(vec![WordToken {
                    // jyutping: "hoeng1gong2jan4".to_owned(),
                    // pos: "n".to_owned(),
                // }])),
            // ),
            // (
                // "講".to_owned(),
                // Some(OutputToken::Word(vec![WordToken {
                    // jyutping: "gong2".to_owned(),
                    // pos: "v".to_owned(),
                // }])),
            // ),
            // (
                // "廣東話".to_owned(),
                // Some(OutputToken::Word(vec![WordToken {
                    // jyutping: "gwong2dung1waa2".to_owned(),
                    // pos: "n".to_owned(),
                // }])),
            // ),
        // ]
        // .into_iter()
        // .collect();

        // // assert_eq!(result, expected_result);
        // assert_eq!(true, false);
    // }

    #[test]
    fn should_handle_unknown_input() {
        let mut rscantonese = RsCantonese::default();

        rscantonese.train(&vec![InputToken {
            word: "香港".to_owned(),
            jyutping: "hoeng1gong2".to_owned(),
            pos: "n".to_owned(),
        }]);

        let result = rscantonese.parse("香港사람");

        let expected_result: Result = vec![
            (
                "香港".to_owned(),
                Some(OutputToken::Word(vec![WordToken {
                    jyutping: "hoeng1gong2".to_owned(),
                    pos: "n".to_owned(),
                }])),
            ),
            ("사".to_owned(), None),
            ("람".to_owned(), None),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_handle_punctuation() {
        let rscantonese = RsCantonese::default();

        let result = rscantonese.parse("，？。,?.");

        let expected_result: Result = vec![
            (
                "，".to_owned(),
                Some(OutputToken::Puncutation(PuncutationToken("，".to_owned()))),
            ),
            (
                "？".to_owned(),
                Some(OutputToken::Puncutation(PuncutationToken("？".to_owned()))),
            ),
            (
                "。".to_owned(),
                Some(OutputToken::Puncutation(PuncutationToken("。".to_owned()))),
            ),
            (
                ",".to_owned(),
                Some(OutputToken::Puncutation(PuncutationToken(",".to_owned()))),
            ),
            (
                "?".to_owned(),
                Some(OutputToken::Puncutation(PuncutationToken("?".to_owned()))),
            ),
            (
                ".".to_owned(),
                Some(OutputToken::Puncutation(PuncutationToken(".".to_owned()))),
            ),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected_result);
    }

    // // #[test]
    // // fn should_get_coverage() {
    // // let mut rscantonese = RsCantonese::default();
    // // rscantonese.train(&wordshk());

    // // let result = rscantonese.characters_to_jyutping("香港사람");

    // // let coverage = RsCantonese::get_coverage(&result);

    // // assert_eq!(coverage, 0.5);
    // // }
}
