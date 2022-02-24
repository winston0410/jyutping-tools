pub mod token;
use token::Token;
pub mod meta;
use meta::Meta;
mod predicates;

use nom::branch::alt;
use nom::IResult;

pub struct MetaToken {
    encoding: String
}

pub struct CorpusToken {}

enum ParserToken {
    Meta(MetaToken),
    Corpus(CorpusToken),
}

pub struct Parser {
    meta: ParserToken,
    data: ParserToken,
}

// pub struct Parser {
// meta: Meta,
// data: Vec<Token>,
// }

impl Parser {
    pub fn parse(raw_input: &str) -> () {
        let result = alt((Token::parse, Meta::parse))(raw_input);

        println!("check result {:?}", result);
    }
}
