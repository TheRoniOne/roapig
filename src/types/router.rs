use super::{comment::Comment, function::Function};

pub type Router<'a, 'b> = (Comment<'a>, Function<'b>);
