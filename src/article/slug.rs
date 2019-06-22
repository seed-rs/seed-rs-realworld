use std::borrow::{Borrow, Cow};

#[derive(Clone)]
pub struct Slug<'a>(Cow<'a, str>);

impl<'a> Slug<'a> {
    pub fn as_str(&'a self) -> &'a str {
        self.0.borrow()
    }
}

impl<'a, T> From<T> for Slug<'a>
    where T: Into<Cow<'a, str>>
{
    fn from(value: T) -> Slug<'a> {
        Slug(value.into())
    }
}