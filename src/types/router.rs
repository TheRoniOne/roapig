use super::function::Function;

#[derive(Debug, PartialEq)]
pub struct Router<'a> {
    pub comment: &'a str,
    pub function: Function<'a>,
}
