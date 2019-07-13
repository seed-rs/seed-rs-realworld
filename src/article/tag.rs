use std::fmt;

#[derive(Clone)]
pub struct Tag(String);

impl Tag {
    pub fn new(tag: String) -> Self {
        Tag(tag)
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}