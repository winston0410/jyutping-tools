use std::collections::HashSet;

pub fn handle_punctuations((key, jyutping): (String, Vec<String>)) -> (String, Vec<String>) {
    let punctuations: HashSet<&str> = HashSet::from([
        // Cantonese punctuations
        "，", "！", "？", "；", "：", "（", "）", "［", "］", "【", "】", "。", "『", "』", "「",
        "」", "、", "·", "《", "》", "〈", "〉", "…", "～", "—",
        // ASC punctuations
        ",", "!", "?", ";", ":", "(", ")", "[", "]", ".", "`", "<", ">", "~", "-"
    ]);

    if punctuations.contains(&*key) {
        (key.to_owned(), vec![key])
    } else {
        (key, jyutping)
    }
}

//REF https://stackoverflow.com/questions/65549983/trait-borrowstring-is-not-implemented-for-str
