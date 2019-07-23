use newtype::NewType;

#[derive(NewType, Clone)]
pub struct Markdown(String);
