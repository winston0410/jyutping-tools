mod token;
use token::Token;
mod meta;
use meta::Meta;
mod predicates;

use nom::branch::alt;
use nom::IResult;

pub struct Parser {}

impl Parser {
    pub fn parse(raw_input: &str) -> () {
        // let result = alt((Token::parse, Meta::parse))(raw_input);
    }
}
