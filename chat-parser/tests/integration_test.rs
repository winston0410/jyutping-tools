#[cfg(test)]
mod tests {
    use chat_parser::corpus::Corpus;
    use chat_parser::Corpora;

    #[test]
    fn should_parse_file() {
        let raw_data = "@UTF8
@Begin
*XXB:	係 ?
%mor:	v|hai6 ?
*XXB:	福利 又 高 喇 .
%mor:	n|fuk1lei6 d|jau6 a|gou1 y|laa1 .
@End";
        let parsed = Corpora::parse(raw_data);

        assert_eq!(parsed.meta.encoding, "UTF8".to_owned());
        assert_eq!(
            parsed.data,
            vec![
                Corpus {
                    word: "係".to_owned(),
                    jyutping: "hai6".to_owned(),
                    pos: "v".to_owned()
                },
                Corpus {
                    word: "福利".to_owned(),
                    jyutping: "fuk1lei6".to_owned(),
                    pos: "n".to_owned()
                },
                Corpus {
                    word: "又".to_owned(),
                    jyutping: "jau6".to_owned(),
                    pos: "d".to_owned()
                },
                Corpus {
                    word: "高".to_owned(),
                    jyutping: "gou1".to_owned(),
                    pos: "a".to_owned()
                },
                Corpus {
                    word: "喇".to_owned(),
                    jyutping: "laa1".to_owned(),
                    pos: "y".to_owned()
                }
            ]
        );
    }

    #[test]
    fn should_parse_hkcancor() {
        let raw_data = "@UTF8\n@Begin\n@Languages:\tyue , eng\n@Participants:\tXXA A Adult , XXB B Adult\n@ID:\tyue , eng|HKCanCor|XXA|23;|female|||Adult||origin:HK|\n@ID:\tyue , eng|HKCanCor|XXB|23;|female|||Adult||origin:HK|\n@Date:\t20-MAY-1997\n@Tape Number:\t011\n*XXA:\t不如 繼續 頭先 講 嘅 嘢 喇 .\n%mor:\tv|bat1jyu4 v|gai3zuk6 t|tau4sin1 v|gong2 u|ge3 n|je5 y|laa1\n\t.\n@End";

        let parsed = Corpora::parse(raw_data);

        assert_eq!(parsed.meta.encoding, "UTF8".to_owned());
        assert_eq!(
            parsed.data,
            vec![
                Corpus {
                    word: "不如".to_owned(),
                    jyutping: "bat1jyu4".to_owned(),
                    pos: "v".to_owned()
                },
                Corpus {
                    word: "繼續".to_owned(),
                    jyutping: "gai3zuk6".to_owned(),
                    pos: "v".to_owned()
                },
                Corpus {
                    word: "頭先".to_owned(),
                    jyutping: "tau4sin1".to_owned(),
                    pos: "t".to_owned()
                },
                Corpus {
                    word: "講".to_owned(),
                    jyutping: "gong2".to_owned(),
                    pos: "v".to_owned()
                },
                Corpus {
                    word: "嘅".to_owned(),
                    jyutping: "ge3".to_owned(),
                    pos: "u".to_owned()
                },
                Corpus {
                    word: "嘢".to_owned(),
                    jyutping: "je5".to_owned(),
                    pos: "n".to_owned()
                },
                Corpus {
                    word: "喇".to_owned(),
                    jyutping: "laa1".to_owned(),
                    pos: "y".to_owned()
                }
            ]
        );
    }
}
