use std::borrow::Cow;

pub struct Slug<'a>(Cow<'a, str>);

impl<'a, T> From<T> for Slug<'a>
    where T: Into<Cow<'a, str>>
{
    fn from(value: T) -> Slug<'a> {
        Slug(value.into())
    }
}