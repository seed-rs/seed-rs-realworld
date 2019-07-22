use std::borrow::{Borrow, Cow};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Slug(Cow<'static, str>);

impl Slug {
    pub fn as_str(&self) -> &str {
        self.0.borrow()
    }
}

impl<T> From<T> for Slug
where
    T: Into<Cow<'static, str>>,
{
    fn from(value: T) -> Slug {
        Slug(value.into())
    }
}
