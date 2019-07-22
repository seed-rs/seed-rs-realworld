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

pub trait IntoStrings {
    fn into_strings(self) -> Vec<String>;
}

impl IntoStrings for Vec<Tag> {
    fn into_strings(self) -> Vec<String> {
        self.into_iter().map(|tag| tag.to_string()).collect()
    }
}

pub trait IntoTags {
    fn into_tags(self) -> Vec<Tag>;
}

impl IntoTags for Vec<String> {
    fn into_tags(self) -> Vec<Tag> {
        self.into_iter().map(Tag::new).collect()
    }
}
