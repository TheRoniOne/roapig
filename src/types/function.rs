#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    pub name: &'a str,
    pub body: &'a str,
}
