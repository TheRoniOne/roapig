use nom::{
    bytes::complete::{tag, take_until},
    sequence::{delimited, tuple},
    IResult,
};

use super::body::parse_c_style_body;

pub fn parse_golang_style_structure_name(input: &str) -> IResult<&str, &str> {
    delimited(tag("type "), take_until(" struct"), take_until("{"))(input)
}

pub fn parse_golang_style_structure(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((parse_golang_style_structure_name, parse_c_style_body))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_golang_style_structure_name() {
        let input = "type MyStruct struct {}";
        let expected = "MyStruct";
        assert_eq!(
            parse_golang_style_structure_name(input).unwrap().1,
            expected
        );
    }

    #[test]
    fn test_parse_golang_style_structure() {
        let input = "type MyStruct struct {
    Field int64;
}";
        let expected = ("MyStruct", "\n    Field int64;");
        assert_eq!(parse_golang_style_structure(input).unwrap().1, expected);
    }
}
