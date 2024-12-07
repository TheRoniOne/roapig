use nom::{
    bytes::complete::{tag, take_until},
    sequence::{delimited, tuple},
    IResult,
};

use crate::types::function::{Function, FunctionName};

use super::body::parse_c_style_body;

fn parse_golang_style_function_name(input: &str) -> IResult<&str, FunctionName> {
    delimited(tag("func "), take_until("("), take_until("{"))(input)
}

pub fn parse_golang_style_function(input: &str) -> IResult<&str, Function> {
    tuple((parse_golang_style_function_name, parse_c_style_body))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_golang_style_function_name() {
        let input = "func main() {}";
        let expected = "main";
        assert_eq!(parse_golang_style_function_name(input).unwrap().1, expected);
    }

    #[test]
    fn test_parse_golang_style_function() {
        let input = "func main() {
    var = 1;
}";
        let expected = ("main", "\n    var = 1;");
        assert_eq!(parse_golang_style_function(input).unwrap().1, expected);
    }
}
