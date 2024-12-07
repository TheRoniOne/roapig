use nom::{
    bytes::complete::{tag, take_until},
    sequence::{delimited, tuple},
    IResult,
};

use super::{comment, function};

pub fn parse_golang_echo_style_router(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    tuple((
        comment::parse_c_style_comment,
        function::parse_golang_style_function,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_golang_echo_style_router() {
        let input = "// ROUTER
func SetUpRoutes() {
    group := app.Group(\"/api\")
}";
        let expected = (
            "ROUTER",
            ("SetUpRoutes", "\n    group := app.Group(\"/api\")"),
        );
        assert_eq!(parse_golang_echo_style_router(input).unwrap().1, expected);
    }
}
