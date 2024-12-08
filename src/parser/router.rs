use nom::{sequence::tuple, IResult};

use crate::types::router::Router;

use super::{comment, function};

pub fn parse_golang_echo_style_router(input: &str) -> IResult<&str, Router> {
    let (input, found) = tuple((
        comment::parse_c_style_comment,
        function::parse_golang_style_function,
    ))(input)?;

    Ok((
        input,
        Router {
            comment: found.0,
            function: found.1,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::types::function::Function;

    use super::*;

    #[test]
    fn test_parse_golang_echo_style_router() {
        let input = "// ROUTER
func SetUpRoutes() {
    group := app.Group(\"/api\")
}";
        let expected = Router {
            comment: "ROUTER",
            function: Function {
                name: "SetUpRoutes",
                body: "\n    group := app.Group(\"/api\")",
            },
        };
        assert_eq!(parse_golang_echo_style_router(input).unwrap().1, expected);
    }
}
