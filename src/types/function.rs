use super::body::Body;

pub type FunctionName<'a> = &'a str;
pub type Function<'a> = (FunctionName<'a>, Body<'a>);
