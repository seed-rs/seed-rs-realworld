use std::fmt;

// ------ Tag ------

#[derive(Clone)]
pub struct Tag(String);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

// ------ IntoStrings ------

pub trait IntoStrings {
    fn into_strings(self) -> Vec<String>;
}

impl IntoStrings for Vec<Tag> {
    fn into_strings(self) -> Vec<String> {
        self.into_iter().map(|tag| tag.0).collect()
    }
}

// ------ IntoTags ------

pub trait IntoTags {
    fn into_tags(self) -> Vec<Tag>;
}

impl IntoTags for Vec<String> {
    fn into_tags(self) -> Vec<Tag> {
        self.into_iter().map(Tag).collect()
    }
}
