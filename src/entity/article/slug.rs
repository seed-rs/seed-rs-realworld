use newtype::NewType;

#[derive(NewType, Clone, Default, PartialEq, Eq)]
pub struct Slug(String);
