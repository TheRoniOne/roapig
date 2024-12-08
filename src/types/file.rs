use super::{function::Function, router::Router, structure::Structure};

#[derive(Debug, PartialEq)]
pub struct File<'a> {
    pub module_name: &'a str,
    pub router: Option<Router<'a>>,
    pub structure: Vec<Structure<'a>>,
    pub handlers: Vec<Function<'a>>,
}
