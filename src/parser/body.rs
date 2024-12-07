use nom::{
    bytes::complete::take_until,
    character::complete::{char, newline},
    sequence::{delimited, pair},
    IResult,
};

use crate::types::body::Body;

pub fn parse_c_style_body(input: &str) -> IResult<&str, Body> {
    delimited(char('{'), take_until("\n}"), pair(newline, char('}')))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_c_style_body() {
        let input = "{
    var = 1;
}
        ";
        let expected = "\n    var = 1;";
        assert_eq!(parse_c_style_body(input).unwrap().1, expected);
    }

    #[test]
    fn test_parse_c_style_body_nested() {
        let input = "{
    var = 1;
    {
        var2 = 2;
    }
}
        ";
        let expected = "\n    var = 1;\n    {\n        var2 = 2;\n    }";
        assert_eq!(parse_c_style_body(input).unwrap().1, expected);
    }
}
