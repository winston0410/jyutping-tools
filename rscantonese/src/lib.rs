use std::collections::HashMap;
use wordseg::Segmenter;

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
                (
                    word.to_owned(),
                    self.conversion_dict.get(&word).unwrap().to_owned(),
                )
            })
            .collect();

        result
    }

    /// Segment Cantonese characters
    pub fn segment(&self, unsegmented: &str) -> Vec<String> {
        self.segmenter.predict(unsegmented)
    }
}

impl Default for RsCantonese {
    /// Generate an instance of RsCantonese, loaded with data from words.hk
    fn default() -> Self {
        let data = include_str!("../data/words.hk/wordslist.json");
        let deserialized: HashMap<String, Vec<String>> = serde_json::from_str(&data).unwrap();

        //REF https://stackoverflow.com/questions/56724014/how-do-i-collect-the-values-of-a-hashmap-into-a-vector
        let keys = deserialized.keys().cloned().collect::<Vec<String>>();
        let mut segmenter = Segmenter::default();
        segmenter.fit(&keys);

        //Fix format of jyutping in dataset, from hoeng1 gong2 to hoeng1gong2
        let formatted = deserialized
            .into_iter()
            .map(|(key, value)| {
                (
                    key,
                    value
                        .into_iter()
                        .map(|mut s| -> String {
                            s.retain(|c| !c.is_whitespace());
                            s
                        })
                        .collect(),
                )
            })
            .collect();

        //TODO Provide option for using specific dataset
        RsCantonese {
            segmenter,
            conversion_dict: formatted,
        }
    }
}
