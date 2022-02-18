use nom::*;
pub mod chat_data;
use chat_data::{ChatData, ChatEncoding, ChatMeta};

pub struct Parser {}

impl Parser {
    pub fn parse(&self, raw_input: &str) -> ChatData {
        ChatData {
            meta: ChatMeta {
                encoding: ChatEncoding::Utf8,
            },
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser {}
    }
}
