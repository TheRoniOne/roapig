use super::body::Body;

#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub body: Body<'a>,
}
