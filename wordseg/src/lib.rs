use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Segmenter {
    pub max_word_length: usize,
    model: HashSet<String>,
}

impl<'a> Segmenter {
    /// Train the model by inputting words fragment. Duplication expected
    pub fn fit(&'a mut self, segmented: &[String]) -> &'a mut Self {
        for word in segmented.iter() {
            if word.graphemes(true).count() > 1 {
                self.model.insert(word.to_owned());
            }
        }

        self
    }

    /// Insert unhandled text for getting prediction
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
                j -= 1;
            };
        }

        result
    }

    /// Update the max_word_length constraint based on the longest token found in model
    pub fn update_constraint(&'a mut self) -> &'a mut Self {
        let longest_length = self.model.iter().fold(0, |acc, item| {
            let length = item.graphemes(true).count();

            if length > acc {
                length
            } else {
                acc
            }
        });

        self.max_word_length = longest_length;
        self
    }
}
