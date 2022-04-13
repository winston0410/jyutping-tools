#[cfg(test)]
mod tests {
    #[test]
    fn should_train() {
        let rscantonese = rscantonese::RsCantonese::default();
        rscantonese.train();

        assert_eq!(true, false)
    }
}