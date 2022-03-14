use std::collections::HashMap;
use wordseg::Segmenter;
use chat_parser::corpus::Corpus;
pub mod data;
pub mod transformer;

#[derive(Default)]
pub struct RsCantonese {
    pub segmenter: Segmenter,
    pub conversion_dict: HashMap<String, Corpus>,
}

impl RsCantonese {
    // Reason why we use Into and AsRef
    //REF https://discord.com/channels/442252698964721669/947648480799752192
    /// Convert cantonese characters to jyutping
    // pub fn characters_to_jyutping<T>(&self, unsegmented: T) -> Vec<(String, Vec<String>)>
    // where
        // T: AsRef<str> + Into<String>,
    // {
        // let segmented = self.segmenter.predict(unsegmented);

        // // Need to handle punctuation here, as their length is one only
        // let result: Vec<(String, Vec<String>)> = segmented
            // .into_iter()
            // .map(|word| -> (String, Vec<String>) {
                // let jyutpings = self
                    // .conversion_dict
                    // .get(&word)
                    // .unwrap_or(&vec!["unknown".to_owned()])
                    // .to_owned();
                // (word, jyutpings)
            // })
            // .map(transformer::handle_punctuations)
            // .collect();

        // result
    // }

    /// Segment Cantonese characters
    pub fn segment<T>(&self, unsegmented: T) -> Vec<String>
    where
        T: AsRef<str> + Into<String>,
    {
        self.segmenter.predict(unsegmented)
    }

    /// Train the model
    pub fn train(&mut self, data: &[Corpus]) -> &Self {
        // handle the vector here
        // println!("{:?}", data);
        
        //TODO Filter out all characters with xn, dont accept foreign words
        
        //Insert vector into hashmap correctly
        
        // self.conversion_dict.extend(data.to_owned());

        // self.segmenter
            // .fit(self.conversion_dict.keys())
            // .update_constraint();
        self
    }
}
