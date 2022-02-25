#[cfg(test)]
mod tests {
    use chat_parser::corpus::Corpus;
    use chat_parser::Corpora;

    #[test]
    fn should_parse_metadata() {
        let raw_data = "@UTF8
@Begin
*XXB:	係 ?
%mor:	v|hai6 ?
*XXB:	福利 又 高 喇 .
%mor:	n|fuk1lei6 d|jau6 a|gou1 y|laa1 .
@End";
        let parsed = Corpora::parse(raw_data);

        assert_eq!(parsed.meta.encoding.to_owned(), "UTF8".to_owned());
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
}
