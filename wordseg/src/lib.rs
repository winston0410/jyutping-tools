use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub struct Segmenter {
    max_word_length: usize,
    model: HashSet<String>,
}

impl Segmenter {
    //REF https://stackoverflow.com/questions/28469667/borrowed-value-does-not-live-long-enough-when-using-the-builder-pattern
    // Have to return orignal copy to avoid reference error

    /// Set the maximum word length for segmentation. The maximum word length cannot be smaller
    /// than 2, as it will be meaningless for segmentation
    pub fn max_word_length(mut self, max_word_length: usize) -> Segmenter {
        if max_word_length < 2 {
            panic!("max_word_length must be >= 2 to be meaningful")
        }
        self.max_word_length = max_word_length;
        self
    }

    /// Train the model by inputting existing result
    pub fn fit(&mut self, segmented: Vec<Vec<&str>>) {
        for s in segmented.into_iter() {
            for word in s.into_iter() {
                if word.graphemes(true).count() > 1 {
                    self.model.insert(word.to_owned());
                }
            }
        }
    }

    /// Insert unhanlded text for getting prediction
    pub fn predict(&self, unsegmented: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let unicoded = unsegmented.graphemes(true);
        let input_length = unicoded.to_owned().count();
        let max_word_length = std::cmp::min(self.max_word_length, input_length);

        // i pointer match the beginning indice of a word
        let mut i: usize = 0;
        // j pointer match the end indice of a word
        let mut j: usize = i + max_word_length;

        while i < input_length {
            let segment = unicoded.to_owned().skip(i).take(j - i).collect::<String>();
            if self.model.contains(&segment) || j - i == 1 {
                result.push(segment.to_owned());
                i = j;
                j = i + max_word_length;
            } else {
                j = j - 1
            };
        }

        return result;
    }
}

impl Default for Segmenter {
    fn default() -> Self {
        Segmenter {
            max_word_length: 2,
            model: HashSet::new(),
        }
    }
}
