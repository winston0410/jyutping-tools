mod token;
use token::Token;
mod meta;
use meta::Meta;
mod predicates;

use nom::branch::alt;
use nom::IResult;

// enum ParserToken {
// Meta { encoding: String },
// Data (),
// }

// pub struct Parser {
// // Should expect ParserToken::meta
// meta: ParserToken,
// // Should expect ParserToken::data
// data: ParserToken,
// }
//

pub struct Parser {
    meta: Meta,
    data: Vec<Token>,
}

impl Parser {
    pub fn parse(raw_input: &str) -> () {
        // let result = alt((Token::parse, Meta::parse))(raw_input);
    }
}
