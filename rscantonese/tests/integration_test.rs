#[cfg(test)]
mod tests {
    #[test]
    fn should_segment_correctly() {
        // let token = std::env::var("GITHUB_TOKEN").unwrap();

        // let fetcher = rscantonese::utils::GitHubFetcher::new(&token);

        // let assets_path = &fetcher.get("https://raw.githubusercontent.com/winston0410/nlp-data/develop/wikidump.tar.gz");

        let rscantonese = rscantonese::RsCantoneseConfig::default().train();
        let result = rscantonese.segment("我係香港人。");

        assert_eq!(result, ["我", "係", "香港人", "。"]);

        assert_eq!(true, false);
    }
}
