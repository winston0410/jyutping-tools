pub mod chat_data;
use chat_data::{ChatData, ChatEncoding, ChatMeta, Token};
mod parser;
use nom::sequence::separated_pair;

pub struct Parser {}

impl Parser {
    // pub fn parse(&self, raw_input: &str) -> IResult<&str, ChatData> {
    pub fn parse(&self, raw_input: &str) -> () {
        // For parse two lines at a time
        // delimited(Token::parse, tag("\n"), second_line_parser)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser {}
    }
}
