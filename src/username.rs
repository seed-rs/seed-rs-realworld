use std::borrow::Cow;
use serde::Deserialize;

#[derive(Eq, PartialEq, Clone, Debug, Deserialize)]
pub struct Username<'a>(Cow<'a, str>);

impl<'a> Username<'a> {
    pub fn new<T>(username: T) -> Self
    where T: Into<Cow<'a, str>>
    {
        Username(username.into())
    }

    pub fn as_str(&'a self) -> &'a str {
        self.as_str()
    }
}

impl<'a, T> From<T> for Username<'a>
    where T: Into<Cow<'a, str>>
{
    fn from(value: T) -> Username<'a> {
        Username(value.into())
    }
}