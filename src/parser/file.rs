use crate::types::file::File;

use super::{
    module_name::parse_golang_style_module_name, router::parse_golang_echo_style_router,
    structure::parse_golang_style_structure,
};
use nom::{branch::alt, IResult};

fn parse_golang_echo_style_file(input: &str) -> IResult<&str, File> {
    let (input, module_name) = parse_golang_style_module_name(input)?;

    todo!()
}
