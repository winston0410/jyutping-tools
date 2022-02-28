use std::collections::HashMap;
use wordseg::Segmenter;
pub mod data;

#[derive(Default)]
pub struct RsCantonese {
    pub segmenter: Segmenter,
    pub conversion_dict: HashMap<String, Vec<String>>,
}

impl RsCantonese {
    /// Convert cantonese characters to jyutping
    pub fn characters_to_jyutping(&self, unsegmented: &str) -> Vec<(String, Vec<String>)> {
        let segmented = self.segmenter.predict(unsegmented);

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
            .collect();

        result
    }

    /// Segment Cantonese characters
    pub fn segment(&self, unsegmented: &str) -> Vec<String> {
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
}
