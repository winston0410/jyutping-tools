use tokenizers::decoders::DecoderWrapper;
use tokenizers::models::bpe::BpeTrainer;
use tokenizers::models::bpe::BPE;
use tokenizers::normalizers::BertNormalizer;
use tokenizers::normalizers::NormalizerWrapper;
use tokenizers::pre_tokenizers::bert::BertPreTokenizer;
use tokenizers::pre_tokenizers::PreTokenizerWrapper;
use tokenizers::processors::PostProcessorWrapper;
use tokenizers::tokenizer::{AddedToken, EncodeInput, Tokenizer};
use tokenizers::TokenizerBuilder;
use tokenizers::decoders::bpe::BPEDecoder;
use tokenizers::processors::bert::BertProcessing;

pub struct RsCantoneseConfig {
    /// Path for saving the trained artifect. Default to `$TMP_DIR/rscantonese.json`
    model_path: std::path::PathBuf,
    vocab_size: usize,
    files: Vec<String>
}

impl RsCantoneseConfig {
    pub fn train(&self) -> RsCantonese {
        let mut trainer = BpeTrainer::builder()
            .vocab_size(self.vocab_size)
            .special_tokens(vec![AddedToken::from("UNK", true)])
            .build();

        let mut tokenizer_builder = TokenizerBuilder::<
            BPE,
            NormalizerWrapper,
            PreTokenizerWrapper,
            PostProcessorWrapper,
            DecoderWrapper,
        >::new()
        .with_model(BPE::default())
        .with_normalizer(Some(NormalizerWrapper::Sequence(
            tokenizers::normalizers::utils::Sequence::new(vec![NormalizerWrapper::BertNormalizer(
                BertNormalizer::new(true, true, None, true),
            )]),
        )))
        .with_pre_tokenizer(Some(PreTokenizerWrapper::Sequence(
            tokenizers::pre_tokenizers::sequence::Sequence::new(vec![
                PreTokenizerWrapper::BertPreTokenizer(BertPreTokenizer),
            ]),
        )))
        .with_post_processor(Some(PostProcessorWrapper::Bert(BertProcessing::default())))
        .with_decoder(Some(
            DecoderWrapper::BPE(BPEDecoder::default())
        ))
        .build()
        .unwrap();

        // let files = vec!["./data/wikidump/wikitext.raw".into()];

        tokenizer_builder
            .train_from_files(&mut trainer, self.files.to_owned())
            .unwrap()
            .save(&self.model_path, false)
            .unwrap();

        let tokenizer = Tokenizer::from_file(&self.model_path).unwrap();

        RsCantonese { tokenizer }
    }
}

impl Default for RsCantoneseConfig {
    fn default() -> Self {
        let mut model_path = std::env::temp_dir();
        model_path.push("rscantonese.json");

        RsCantoneseConfig { model_path, vocab_size: 200000, files: Vec::new() }
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