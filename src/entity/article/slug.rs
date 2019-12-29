use newtype::NewType;

#[derive(NewType, Clone, Default, PartialEq, Eq, Debug)]
pub struct Slug(String);
