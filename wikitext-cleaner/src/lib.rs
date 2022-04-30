
use nom::branch::permutation;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;

use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many0;

use nom::sequence::delimited;

use nom::sequence::terminated;
use nom::IResult;
use regex::Regex;

fn match_doc_tag(input: &str) -> IResult<&str, &str> {
    recognize(permutation((tag("<doc"), take_until(">"), tag(">"))))(input)
}

#[cfg(test)]
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

///Remove parenthesis in sentence
/// 
/// Input:
/// ```
/// 香港（，）係華南一城埠。
/// ```
/// 
/// Output:
/// ```
/// 香港係華南一城埠。
/// ```
pub fn remove_parenthesis<T>(input: T) -> String 
    where T: AsRef<str> + Into<String> 
{
    let regex = Regex::new(r"（.*）").unwrap();
    regex.replace_all(input.as_ref(), "").to_string()
}

#[cfg(test)]
mod test_remove_parenthesis {
    use super::*;

    #[test]
    fn should_remove_parenthesis() {
        let raw = "香港（，）係華南一城埠。";
        let replaced = remove_parenthesis(raw);

        assert_eq!(replaced, "香港係華南一城埠。");
    }
}

///Add linebreak based on footstop.
/// 
/// Input:
/// ```
/// 香港係華南一城埠。香港開埠於1841年。
/// ```
/// 
/// Output:
/// ```
/// 香港係華南一城埠。\n香港開埠於1841年。\n
/// ```
pub fn linebreak_based_on_footstop<T>(input: T) -> String 
where T: AsRef<str> + Into<String> {
    input.as_ref().replace("。", "。\n")
}

#[cfg(test)]
mod test_linebreak_based_on_footstop {
    use super::*;

    #[test]
    fn should_linebreak_based_on_footstop() {
        let raw = "香港係華南一城埠。香港開埠於1841年，有人叫佢做「百年之城」。";
        let replaced = linebreak_based_on_footstop(raw);
    
        assert_eq!(replaced, "香港係華南一城埠。\n香港開埠於1841年，有人叫佢做「百年之城」。\n");
    }
}

///Remove excessive linebreak.
/// 
/// Input:
/// ```
/// \n\n
/// ```
/// 
/// Output:
/// ```
/// \n
/// ```
pub fn remove_excessive_linebreak<T>(input: T) -> String 
where T: AsRef<str> + Into<String> {
    let regex = Regex::new(r"\n+").unwrap();
    regex.replace_all(input.as_ref(), "\n").to_string()
}

#[cfg(test)]
mod test_remove_excessive_linebreak {
    use super::*;
    #[test]
    fn should_excessive_linebreak() {
        let raw = "香港\n\n香港係華南一城埠。";
        let replaced = remove_excessive_linebreak(raw);
        
        assert_eq!(replaced, "香港\n香港係華南一城埠。");
    }
}

//Should we remove all English?
// pub fn remove_english<T>(input: T) -> String 
// where T: AsRef<str> + Into<String> {
//     let regex = Regex::new(r"\n+").unwrap();
//     regex.replace_all(input.as_ref(), "\n").to_string()
// }

// #[cfg(test)]
// mod test_remove_english {
//     use super::*;
//     #[test]
//     fn should_remove_english() {
//         let raw = "我去咗Central London食飯。";
//         let replaced = remove_english(raw);
        
//         assert_eq!(replaced, "我去咗食飯。");
//     }
// }