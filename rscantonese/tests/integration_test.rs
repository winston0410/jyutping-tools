#[cfg(test)]
mod integrated_tests {
    use rscantonese::token::{InputToken, OutputToken};
    use rscantonese::{RsCantonese, Result};

    #[test]
    fn should_convert_cantonese_to_jyutping() {
        let mut rscantonese = RsCantonese::default();

        rscantonese.train(&vec![
            InputToken {
                word: "香港人".to_owned(),
                jyutping: "hoeng1gong2jan4".to_owned(),
                pos: "n".to_owned(),
            },
            InputToken {
                word: "講".to_owned(),
                jyutping: "gong2".to_owned(),
                pos: "v".to_owned(),
            },
            InputToken {
                word: "廣東話".to_owned(),
                jyutping: "gwong2dung1waa2".to_owned(),
                pos: "n".to_owned(),
            },
        ]);

        let result = rscantonese.parse("香港人講廣東話");

        let expected_result: Result = vec![
            (
                "香港人".to_owned(),
                Some(vec![OutputToken {
                    jyutping: "hoeng1gong2jan4".to_owned(),
                    pos: "n".to_owned(),
                }]),
            ),
            (
                "講".to_owned(),
                Some(vec![OutputToken {
                    jyutping: "gong2".to_owned(),
                    pos: "v".to_owned(),
                }]),
            ),
            (
                "廣東話".to_owned(),
                Some(vec![OutputToken {
                    jyutping: "gwong2dung1waa2".to_owned(),
                    pos: "n".to_owned(),
                }]),
            ),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected_result);
    }

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
                Some(vec![OutputToken {
                    jyutping: "hoeng1gong2".to_owned(),
                    pos: "n".to_owned(),
                }]),
            ),
            ("사".to_owned(), None),
            ("람".to_owned(), None),
        ]
        .into_iter()
        .collect();

        assert_eq!(result, expected_result);
    }

    // #[test]
    // fn should_handle_punctuation() {
    // let rscantonese = RsCantonese::default();

    // let result = rscantonese.characters_to_jyutping("，？。,?.");

    // let expected_result: Vec<(String, Vec<String>)> = vec![
    // ("，", vec!["，"]),
    // ("？", vec!["？"]),
    // ("。", vec!["。"]),
    // (",", vec![","]),
    // ("?", vec!["?"]),
    // (".", vec!["."]),
    // ]
    // .into_iter()
    // .map(structure_result)
    // .collect();

    // assert_eq!(result, expected_result);
    // }

    // #[test]
    // fn should_get_coverage() {
    // let mut rscantonese = RsCantonese::default();
    // rscantonese.train(&wordshk());

    // let result = rscantonese.characters_to_jyutping("香港사람");

    // let coverage = RsCantonese::get_coverage(&result);

    // assert_eq!(coverage, 0.5);
    // }
}
