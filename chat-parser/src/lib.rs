pub mod corpus;
pub mod meta;
mod predicates;
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;

#[derive(Debug)]
pub enum Token {
    Meta((String, Option<String>)),
    Corpus(Vec<corpus::Corpus>),
}

#[derive(Default, Debug)]
pub struct Corpora {
    pub meta: meta::Meta,
    pub data: Vec<corpus::Corpus>,
}

impl Corpora {
    pub fn parse(raw_input: &str) -> Self {
        let parsed = separated_list1(
            line_ending,
            alt((
                map(corpus::Corpus::parse, Token::Corpus),
                map(meta::Meta::parse, |(key, value)| {
                    Token::Meta((key.to_owned(), value.map(String::from)))
                }),
            )),
        )(raw_input);

        //TODO Handle error
        let (_, tokens) = parsed.unwrap();
        let mut result = Corpora::default();

        for token in tokens.into_iter() {
            match token {
                Token::Meta((key, _value)) => match key.as_str() {
                    "UTF8" => {
                        result.meta.encoding = key;
                    }
                    _ => (),
                },
                Token::Corpus(mut items) => {
                    result.data.append(&mut items);
                }
            }
        }

        result
    }
}
