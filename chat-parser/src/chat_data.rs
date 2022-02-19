use std::fmt;
/// Token for containing all data included in the CHAT file
pub struct Token {
    /// Cantonese characters
    word: String,
    /// Jyutping romanization
    jyutping: String,
    /// Part-of-speech
    pos: String,
}

#[derive(Debug)]
pub enum ChatEncoding {
    Utf8,
    Unknown,
}

#[derive(Debug)]
pub struct ChatMeta {
    pub encoding: ChatEncoding,
}

#[derive(Debug)]
pub struct ChatData {
    pub meta: ChatMeta,
}

impl fmt::Display for ChatEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChatEncoding::Utf8 => write!(f, "utf8"),
            ChatEncoding::Unknown => write!(f, "unknown"),
        }
    }
}
