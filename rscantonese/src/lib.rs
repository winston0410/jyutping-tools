use tokenizers::tokenizer::{Result, Tokenizer};

#[derive(Default)]
pub struct RsCantonese {}

impl RsCantonese {
    pub fn train(&self) -> Result<()>{
        let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?;

        let encoding = tokenizer.encode("Hey there!", false)?;
        println!("{:?}", encoding.get_tokens());

        Ok(())
    }
}