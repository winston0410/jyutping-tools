use crate::token::{OutputToken, PuncutationToken};
use std::collections::HashSet;

pub fn handle_punctuations(
    (key, jyutping): (String, Option<OutputToken>),
) -> (String, Option<OutputToken>) {
    let punctuations: HashSet<&str> = HashSet::from([
        // Cantonese punctuations
        "，", "！", "？", "；", "：", "（", "）", "［", "］", "【", "】", "。", "『", "』", "「",
        "」", "、", "·", "《", "》", "〈", "〉", "…", "～", "—", // ASC punctuations
        ",", "!", "?", ";", ":", "(", ")", "[", "]", ".", "`", "<", ">", "~", "-",
    ]);

    if punctuations.contains(&*key) {
        (
            key.to_owned(),
            Some(OutputToken::Puncutation(PuncutationToken(key))),
        )
    } else {
        (key, jyutping)
    }
}

//REF https://stackoverflow.com/questions/65549983/trait-borrowstring-is-not-implemented-for-str
