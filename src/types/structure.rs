use super::body::Body;

pub type StructureName<'a> = &'a str;
pub type Structure<'a> = (StructureName<'a>, Body<'a>);
