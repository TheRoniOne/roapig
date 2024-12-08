#[derive(Debug, PartialEq)]
pub struct Structure<'a> {
    pub name: &'a str,
    pub body: &'a str,
}
