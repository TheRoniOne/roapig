use nom::{
    bytes::complete::tag, character::complete::not_line_ending, sequence::delimited, IResult,
};

use crate::types::module_name::ModuleName;

pub fn parse_golang_style_module_name(input: &str) -> IResult<&str, ModuleName> {
    delimited(tag("package "), not_line_ending, tag("\n"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_golang_style_module() {
        let input = "package main\n";
        let expected = "main";
        assert_eq!(parse_golang_style_module_name(input).unwrap().1, expected);
    }
}
