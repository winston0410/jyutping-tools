use chat_parser::corpus::Corpus;
use chat_parser::Corpora;
use include_dir::{include_dir, Dir};

pub fn hkcancor() -> Vec<Corpus> {
    //TODO check when to use static in Rust?
    static DATA_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/data/hkcancor");
    let mut corpora: Vec<Corpus> = Vec::new();

    // for file in DATA_DIR.find("**/*.cha").unwrap() {
    for file in DATA_DIR.find("**/FC-011_v.cha").unwrap() {
        let content = file.as_file().unwrap().contents_utf8().unwrap();
        println!("{:?}", content);
        //FIXME The parser doesn't parse all characters right now
        let mut parsed = Corpora::parse(&content);

        corpora.append(&mut parsed.data);
    }

    corpora
}
