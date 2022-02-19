pub mod chat_data;
use chat_data::{ChatData, ChatEncoding, ChatMeta};
mod parser;

use nom::sequence::separated_pair;
use nom::IResult;
//REF https://blog.adamchalmers.com/nom-chars/

pub struct Parser {}

impl Parser {
    // pub fn parse(&self, raw_input: &str) -> IResult<&str, ChatData> {
    pub fn parse(&self, raw_input: &str) -> () {}
}

impl Default for Parser {
    fn default() -> Self {
        Parser {}
    }
}
