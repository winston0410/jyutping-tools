#[cfg(test)]
mod tests {
    #[test]
    fn should_convert_cantonese_to_jyutping() {
        let result = rscantonese::characters_to_jyutping("香港人講廣東話");

        assert_eq!(
            result,
            vec![
                ("香港人".to_owned(), "hoeng1gong2jan4".to_owned()),
                ("講".to_owned(), "gong2".to_owned()),
                ("廣東話".to_owned(), "gwong2dung1waa2".to_owned())
            ]
        );
    }
}
