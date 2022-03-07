use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub struct Segmenter {
    pub max_word_length: usize,
    model: HashSet<String>,
}

impl Segmenter {
    /// Train the model by inputting words fragment. Duplication expected
    pub fn fit<T, E>(&mut self, segmented: T) -> &mut Self
    where
        T: IntoIterator<Item = E>,
        E: AsRef<str> + Into<String>,
    {
        for word in segmented {
            let word_str = word.as_ref();
            // take 2 so at most we see 2 graphemes instead of counting
            // every grapheme in the string
            if word_str.graphemes(true).take(2).count() > 1 {
                self.model.insert(word.into());
            }
        }

        self
    }

    /// Insert unhandled text for getting prediction
    pub fn predict<T>(&self, unsegmented: T) -> Vec<String>
    where
        T: AsRef<str> + Into<String>,
    {
        let mut result: Vec<String> = Vec::new();
        let unicoded = unsegmented.as_ref().graphemes(true);
        let input_length = unicoded.to_owned().count();
        let max_word_length = std::cmp::min(self.max_word_length, input_length);

        // i pointer match the beginning indice of a word
        let mut i: usize = 0;
        // j pointer match the end indice of a word
        let mut j: usize = i + max_word_length;

        while i < input_length {
            let segment = unicoded.to_owned().skip(i).take(j - i).collect::<String>();
            if self.model.contains(&segment) || j - i == 1 {
                result.push(segment);
                i = j;
                j = i + max_word_length;
            } else {
                j -= 1;
            };
        }

        result
    }

    /// Update the max_word_length constraint based on the longest token found in model
    pub fn update_constraint(&mut self) -> &mut Self {
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

impl Default for Segmenter {
    fn default() -> Self {
        Self {
            // Use 1 as the default value, in order to allow wordseg run properly without training
            max_word_length: 1,
            model: HashSet::new()
        }
    }
}
