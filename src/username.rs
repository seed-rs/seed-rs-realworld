use std::borrow::Cow;

pub struct Username<'a>(Cow<'a, str>);

impl<'a> Username<'a> {
    pub fn new<T>(username: T) -> Self
    where T: Into<Cow<'a, str>>
    {
        Username(username.into())
    }
}

impl<'a, T> From<T> for Username<'a>
    where T: Into<Cow<'a, str>>
{
    fn from(value: T) -> Username<'a> {
        Username(value.into())
    }
}