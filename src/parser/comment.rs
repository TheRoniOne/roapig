use nom::{
    bytes::complete::tag, character::complete::not_line_ending, sequence::delimited, IResult,
};

use crate::types::comment::Comment;

pub fn parse_c_style_comment(input: &str) -> IResult<&str, Comment> {
    delimited(tag("// "), not_line_ending, tag("\n"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_c_style_comment() {
        let input = "// this is a comment\n";
        let expected = "this is a comment";
        assert_eq!(parse_c_style_comment(input).unwrap().1, expected);
    }

    #[test]
    fn test_parse_c_style_comment_empty() {
        let input = "var = 1";
        let result = parse_c_style_comment(input);
        assert!(result.is_err());
    }
}
