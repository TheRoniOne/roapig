use nom::{
    bytes::complete::tag, character::complete::not_line_ending, sequence::delimited, IResult,
};

pub fn parse_golang_style_module(input: &str) -> IResult<&str, &str> {
    delimited(tag("package "), not_line_ending, tag("\n"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_golang_style_module() {
        let input = "package main\n";
        let expected = "main";
        assert_eq!(parse_golang_style_module(input).unwrap().1, expected);
    }
}
