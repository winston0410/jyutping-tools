// use wordshk_tools::parse::parse_dict;
use crate::token::InputToken;
// use chat_parser::corpus::Corpus;
// use chat_parser::Corpora;
// use include_dir::{include_dir, Dir};

// pub fn hkcancor() -> Vec<Corpus> {
// //TODO check when to use static in Rust?
// static DATA_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data/hkcancor");
// let mut corpora: Vec<Corpus> = Vec::new();

// // for file in DATA_DIR.find("**/*.cha").unwrap() {
// for file in DATA_DIR.find("**/FC-011_v.cha").unwrap() {
// let content = file.as_file().unwrap().contents_utf8().unwrap();
// println!("{:?}", content);
// //FIXME The parser doesn't parse all characters right now
// let mut parsed = Corpora::parse(&content);

// corpora.append(&mut parsed.data);
// }

// corpora
// }
//

// pub fn wordshk() -> () {
    // static DATA_FILE: &'static str = include_str!("../data/words.hk/all.csv");
    // let dict = parse_dict(DATA_FILE.as_bytes());
    // println!("{:?}", dict);
// }

pub fn mock() -> Vec<InputToken> {
    vec![
        InputToken {
            word: "香港人".to_owned(),
            jyutping: "hoeng1gong2jan4".to_owned(),
            pos: "n".to_owned(),
        },
        InputToken {
            word: "講".to_owned(),
            jyutping: "gong2".to_owned(),
            pos: "v".to_owned(),
        },
        InputToken {
            word: "廣東話".to_owned(),
            jyutping: "gwong2dung1waa2".to_owned(),
            pos: "n".to_owned(),
        },
    ]
}
