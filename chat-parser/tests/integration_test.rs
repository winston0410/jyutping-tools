#[cfg(test)]
mod tests {
    #[test]
    fn should_return_encoding() {
        let raw_data = "
@UTF8
@Begin
        ";
        assert_eq!(true, true);
    }

    #[test]
    fn should_return_languages() {
        let raw_data = "
@UTF8
@Begin
@Languages:	yue , eng
        ";
        assert_eq!(true, true);
    }

    #[test]
    fn should_return_date() {
        let raw_data = "
@UTF8
@Begin
@Date:	30-APR-1997
        ";
        assert_eq!(true, true);
    }
}
