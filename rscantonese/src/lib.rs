use chat_parser::Corpora;
use wordseg::Segmenter;

pub struct RsCantonese {
    pub segmenter: Segmenter,
}

impl RsCantonese {
    /// Convert cantonese characters to jyutping
    pub fn characters_to_jyutping(&self, unsegmented: &str) -> Vec<(String, String)> {
        // self.segmenter.predict(unsegmented)
        vec![("hello".to_owned(), "world".to_owned())]
    }

    /// Segment Cantonese characters
    pub fn segment(&self, unsegmented: &str) -> Vec<String> {
        self.segmenter.predict(unsegmented)
    }
}

impl Default for RsCantonese {
    fn default() -> Self {
        // train model here
        RsCantonese {
            segmenter: Segmenter::default(),
        }
    }
}
