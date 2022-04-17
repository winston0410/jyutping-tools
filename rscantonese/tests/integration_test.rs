#[cfg(test)]
mod tests {
    use rust_bert::xlnet::XLNetConfig;
    use rust_bert::xlnet::XLNetForTokenClassification;
    use rust_bert::Config;
    use std::path::Path;
    use tch::nn;
    use tch::Device;

    #[test]
    fn should_segment_correctly() {
        // REF https://docs.rs/rust-bert/latest/rust_bert/xlnet/struct.XLNetForTokenClassification.html
        let config_path = Path::new("./data/config.json");
        let device = Device::Cpu;
        let p = nn::VarStore::new(device);
        let config = XLNetConfig::from_file(config_path);
        let xlnet_model = XLNetForTokenClassification::new(&p.root(), &config);

        // let token = std::env::var("GITHUB_TOKEN").unwrap();

        // let fetcher = rscantonese::utils::GitHubFetcher::new(&token);

        // let assets_path = &fetcher.get("https://raw.githubusercontent.com/winston0410/nlp-data/develop/wikidump.tar.gz");

        // let rscantonese = rscantonese::RsCantoneseConfig::default().train();
        // let result = rscantonese.segment("我係香港人。");

        // assert_eq!(result, ["我", "係", "香港人", "。"]);

        assert_eq!(true, false);
    }
}
