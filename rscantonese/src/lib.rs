use tokenizers::models::bpe::BpeTrainer;
use tokenizers::tokenizer::{EncodeInput, Tokenizer, TokenizerImpl, AddedToken};
use tokenizers::models::bpe::BPE;
use tokenizers::normalizers::NormalizerWrapper;
use tokenizers::pre_tokenizers::PreTokenizerWrapper;
use tokenizers::processors::PostProcessorWrapper;
use tokenizers::decoders::DecoderWrapper;

pub struct RsCantoneseConfig {
    /// Path for saving the trained artifect
    model_path: std::path::PathBuf
}

impl Default for RsCantoneseConfig {
    fn default() -> Self {
        let mut model_path = std::env::temp_dir();
        model_path.push("rscantonese.json");

        RsCantoneseConfig { model_path }
    }
}

impl RsCantoneseConfig {
    pub fn train(&self) -> RsCantonese {
        let mut trainer = BpeTrainer::builder().special_tokens(vec![
            AddedToken::from("[UNK]", true)
        ]).build();

        let mut tokenizerImpl: TokenizerImpl<
            BPE,
            NormalizerWrapper,
            PreTokenizerWrapper,
            PostProcessorWrapper,
            DecoderWrapper,
        > = TokenizerImpl::new(
            BPE::builder()
                .unk_token("[UNK]".to_string())
                .build()
                .unwrap(),
        );

        let files = vec![
            "./data/wikitext.raw".into(),
        ];

        tokenizerImpl.train_from_files(&mut trainer, files).unwrap();

        tokenizerImpl.save(&self.model_path, false).unwrap();

        let tokenizer = Tokenizer::from_file(&self.model_path).unwrap();

        RsCantonese { tokenizer }
    }
}
pub struct RsCantonese {
    tokenizer: Tokenizer,
}

/// Struct that contains the tokenizer. This should be created from RsCantoneseConfig.train()
impl RsCantonese {
    pub fn segment<'s, T>(&self, input: T) -> Vec<String>
    where
        T: Into<EncodeInput<'s>>,
    {
        let encoding = self.tokenizer.encode(input, false).unwrap();
        encoding.get_tokens().to_vec()
    }
}