use nom::branch::permutation;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::sequence::delimited;
use nom::bytes::complete::take_until;
use nom::IResult;
use nom::combinator::recognize;

fn match_doc_tag(input: &str) -> IResult<&str, &str> {
    recognize(permutation((
        tag("<doc"),
        take_until(">"),
        tag(">")
    )))(input)
}

mod test_doc_tag {
    use super::*;

    #[test]
    fn should_match_doc_tag() {
        assert_eq!(match_doc_tag("<doc id=\"2\" url=\"https://zh-yue.wikipedia.org/wiki?curid=2\" title=\"香港\">"), Ok(("", "<doc id=\"2\" url=\"https://zh-yue.wikipedia.org/wiki?curid=2\" title=\"香港\">")));
    }
}

fn remove_xml(input: &str) -> IResult<&str, &str> {
    delimited(
        permutation((match_doc_tag, line_ending)),
        take_until("</doc>"),
        tag("</doc>")
    )(input)
}

#[cfg(test)]
mod test_remove_xml {
    use super::*;

    #[test]
    fn should_remove_all_xml_tags() {
        assert_eq!(remove_xml("<doc id=\"2\" url=\"https://zh-yue.wikipedia.org/wiki?curid=2\" title=\"香港\">\n我係香港人\n</doc>"), Ok(("", "我係香港人")));
    }
}

// fn remove_parenthesis(input: &str) -> IResult<&str, &str>{

// }

// mod test_remove_parenthesis {
//     use super::*;

//     #[test]
//     fn should_remove_parenthesis() {
//         assert_eq!(remove_parenthesis("一國兩制（）"), Ok(("", "一國兩制")));
//     }
// }

pub fn transform(input: &str) -> String{
    let result = remove_xml(input).unwrap();
    result.1.to_owned()
}
