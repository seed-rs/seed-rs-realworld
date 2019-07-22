use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap, Clone, Default, PartialEq, Eq)]
pub struct Slug(String);

impl From<String> for Slug {
    fn from(slug: String) -> Self {
        Self(slug)
    }
}
