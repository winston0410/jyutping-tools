use std::collections::HashMap;

pub fn wordshk() -> HashMap<String, Vec<String>> {
    let data = include_str!("../data/words.hk/wordslist.json");
    let deserialized: HashMap<String, Vec<String>> = serde_json::from_str(&data).unwrap();

    //REF https://stackoverflow.com/questions/56724014/how-do-i-collect-the-values-of-a-hashmap-into-a-vector
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

    formatted
}
