use nom::branch::alt;
use nom::branch::permutation;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;

fn match_doc_tag(input: &str) -> IResult<&str, &str> {
    recognize(permutation((tag("<doc"), take_until(">"), tag(">"))))(input)
}

mod test_doc_tag {
    use super::*;

    #[test]
    fn should_match_doc_tag() {
        assert_eq!(
            match_doc_tag(
                "<doc id=\"2\" url=\"https://zh-yue.wikipedia.org/wiki?curid=2\" title=\"香港\">"
            ),
            Ok((
                "",
                "<doc id=\"2\" url=\"https://zh-yue.wikipedia.org/wiki?curid=2\" title=\"香港\">"
            ))
        );
    }
}

fn remove_xml(input: &str) -> IResult<&str, &str> {
    delimited(
        match_doc_tag,
        take_until("</doc>"),
        tag("</doc>"),
    )(input)
}

#[cfg(test)]
mod test_remove_xml {
    use super::*;

    #[test]
    fn should_remove_all_xml_tags() {
        assert_eq!(remove_xml("<doc id=\"2\" url=\"https://zh-yue.wikipedia.org/wiki?curid=2\" title=\"香港\">\n我係香港人\n</doc>"), Ok(("", "\n我係香港人\n")));
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

pub fn transform(input: &str) -> String {
    let result = many0(terminated(remove_xml, opt(line_ending)))(input).unwrap();

    String::from_iter(result.1)
}

#[cfg(test)]
mod test_transform {
    use super::*;

    #[test]
    fn should_handle_multiple_blocks() {
        assert_eq!(transform("<doc id=\"2\" url=\"https://zh-yue.wikipedia.org/wiki?curid=2\" title=\"香港\">\n我係香港人\n</doc>\n<doc id=\"2\" url=\"https://zh-yue.wikipedia.org/wiki?curid=2\" title=\"香港\">\n我係英國人\n</doc>\n"), "\n我係香港人\n\n我係英國人\n");
    }
}
