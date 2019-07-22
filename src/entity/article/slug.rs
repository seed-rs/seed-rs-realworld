#[derive(Clone, Default, PartialEq, Eq)]
pub struct Slug(String);

impl Slug {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Slug {
    fn from(slug: String) -> Self {
        Self(slug)
    }
}
