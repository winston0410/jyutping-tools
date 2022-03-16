use crate::token::OutputToken;
use std::collections::HashSet;

pub fn handle_punctuations(
    (key, jyutping): (String, Option<Vec<OutputToken>>),
) -> (String, Option<Vec<OutputToken>>) {
    let punctuations: HashSet<&str> = HashSet::from([
        // Cantonese punctuations
        "，", "！", "？", "；", "：", "（", "）", "［", "］", "【", "】", "。", "『", "』", "「",
        "」", "、", "·", "《", "》", "〈", "〉", "…", "～", "—", // ASC punctuations
        ",", "!", "?", ";", ":", "(", ")", "[", "]", ".", "`", "<", ">", "~", "-",
    ]);

    if punctuations.contains(&*key) {
        //FIXME returning None is not useful for handling the mapping. Should make the Hashmap
        //accept enum instead
        (key.to_owned(), None)
    } else {
        (key, jyutping)
    }
}

//REF https://stackoverflow.com/questions/65549983/trait-borrowstring-is-not-implemented-for-str
