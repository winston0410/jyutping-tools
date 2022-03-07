use std::collections::HashMap;
use wordseg::Segmenter;
pub mod data;
pub mod transformer;

#[derive(Default)]
pub struct RsCantonese {
    pub segmenter: Segmenter,
    pub conversion_dict: HashMap<String, Vec<String>>,
    // Accept an array of functions for transforming input
    // This is needed as we cannot handle punctuation with segmenter, they will only have a count() of 1 and are ignored
    // transformer: Vec<String>
}

impl RsCantonese {
    /// Convert cantonese characters to jyutping
    pub fn characters_to_jyutping<T>(&self, unsegmented: T) -> Vec<(String, Vec<String>)>
    where
        T: AsRef<str> + Into<String>,
    {
        let segmented = self.segmenter.predict(unsegmented);

        // Need to handle punctuation here, as their length is one only
        let result: Vec<(String, Vec<String>)> = segmented
            .into_iter()
            .map(|word| -> (String, Vec<String>) {
                let jyutpings = self
                    .conversion_dict
                    .get(&word)
                    .unwrap_or(&vec!["unknown".to_owned()])
                    .to_owned();
                (word, jyutpings)
            })
            .map(transformer::handle_punctuations)
            .collect();

        result
    }

    /// Segment Cantonese characters
    pub fn segment<T>(&self, unsegmented: T) -> Vec<String>
    where
        T: AsRef<str> + Into<String>,
    {
        self.segmenter.predict(unsegmented)
    }

    /// Train the model
    pub fn train(&mut self, data: &HashMap<String, Vec<String>>) -> &Self {
        self.conversion_dict.extend(data.to_owned());

        self.segmenter
            .fit(self.conversion_dict.keys())
            .update_constraint();
        self
    }
    
    // Apply a transformer function to transform the output of rscantonese
    // pub fn apply(&mut self, transformer: fn() -> ()) -> &Self {
        // self
    // }
}
