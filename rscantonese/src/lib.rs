use typed_builder::TypedBuilder;
pub mod utils;
mod constant;

#[derive(TypedBuilder)]
/// Configuration for training RsCantonese transformer
pub struct RsCantoneseConfig {

}

impl RsCantoneseConfig {

}


// pub struct RsCantonese {
//     tokenizer: Tokenizer,
// }

// /// Struct that contains the tokenizer. This should be created from RsCantoneseConfig.train()
// impl RsCantonese {
//     pub fn segment<'s, T>(&self, input: T) -> Vec<String>
//     where
//         T: Into<EncodeInput<'s>>,
//     {
//         let encoding = self.tokenizer.encode(input, false).unwrap();
//         encoding.get_tokens().to_vec()
//     }
// }
