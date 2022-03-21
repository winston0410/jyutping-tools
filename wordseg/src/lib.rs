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

    /// Bidirectional matching rule for segmentation. It will use predict and reverse_predict
    /// interchangeably
    // pub fn bidirectional_predict<T>(&self, unsegmented: T) -> Vec<String>
    // where
    // T: AsRef<str> + Into<String>
    // {

    // }

    pub fn base_predict<T>(&self, unsegmented: T, is_forward: bool) -> Vec<String>
    where
        T: AsRef<str> + Into<String>,
    {
        let mut result: Vec<String> = Vec::new();
        let unicoded = unsegmented.as_ref().graphemes(true);
        let input_length = unicoded.to_owned().count();
        let max_word_length = std::cmp::min(self.max_word_length, input_length);

        let mut i: usize = if is_forward { 0 } else { input_length };
        let mut j: usize = if is_forward {
            i + max_word_length
        } else {
            i - max_word_length
        };

        while if is_forward { i < input_length } else { i > 0 } {
            let segment = unicoded
                .to_owned()
                .skip(if is_forward { i } else { j })
                .take(if is_forward { j - i } else { i - j })
                .collect::<String>();

            if self.model.contains(&segment) || (if is_forward { j - i } else { i - j }) == 1 {
                result.push(segment);
                i = j;
                j = if is_forward {
                    i + max_word_length
                } else {
                    match i.checked_sub(max_word_length) {
                        None => 0,
                        Some(res) => res,
                    }
                }
            } else {
                if is_forward {
                    j -= 1;
                } else {
                    j += 1;
                }
            };
        }

        if !is_forward {
            result.reverse()
        }

        result
    }

    pub fn reverse_predict<T>(&self, unsegmented: T) -> Vec<String>
    where
        T: AsRef<str> + Into<String>,
    {
        self.base_predict(unsegmented, false)
    }

    /// Reverse maximal matching rule for segmentation
    // pub fn reverse_predict<T>(&self, unsegmented: T) -> Vec<String>
    // where
    // T: AsRef<str> + Into<String>,
    // {
    // let mut result: Vec<String> = Vec::new();
    // let unicoded = unsegmented.as_ref().graphemes(true);
    // let input_length = unicoded.to_owned().count();
    // let max_word_length = std::cmp::min(self.max_word_length, input_length);

    // // i is slow pointer and j is fast pointer, from right to left.
    // let mut i: usize = input_length;
    // let mut j: usize = i - max_word_length;

    // while i > 0 {
    // let segment = unicoded.to_owned().skip(j).take(i - j).collect::<String>();

    // if self.model.contains(&segment) || i - j == 1 {
    // result.push(segment);
    // i = j;
    // j = match i.checked_sub(max_word_length) {
    // None => 0,
    // Some(res) => res,
    // };
    // } else {
    // j += 1;
    // };
    // }

    // result.reverse();

    // result
    // }
    //

    /// Forward maximal matching rule for segmentation
    pub fn forward_predict<T>(&self, unsegmented: T) -> Vec<String>
    where
        T: AsRef<str> + Into<String>,
    {
        self.base_predict(unsegmented, true)
    }

    // /// Forward maximal matching rule for segmentation
    // pub fn forward_predict<T>(&self, unsegmented: T) -> Vec<String>
    // where
    // T: AsRef<str> + Into<String>,
    // {
    // let mut result: Vec<String> = Vec::new();
    // let unicoded = unsegmented.as_ref().graphemes(true);
    // let input_length = unicoded.to_owned().count();
    // let max_word_length = std::cmp::min(self.max_word_length, input_length);

    // // i is slow pointer and j is fast pointer, from left to right.
    // let mut i: usize = 0;
    // let mut j: usize = i + max_word_length;

    // while i < input_length {
    // let segment = unicoded.to_owned().skip(i).take(j - i).collect::<String>();
    // if self.model.contains(&segment) || j - i == 1 {
    // result.push(segment);
    // i = j;
    // j = i + max_word_length;
    // } else {
    // j -= 1;
    // };
    // }

    // result
    // }

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
            model: HashSet::new(),
        }
    }
}
