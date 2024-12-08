use super::{comment::Comment, function::Function};

#[derive(Debug, PartialEq)]
pub struct Router<'a, 'b> {
    pub comment: Comment<'a>,
    pub function: Function<'b>,
}
