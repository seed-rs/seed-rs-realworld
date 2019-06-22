use std::borrow::{Borrow, Cow};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct Username<'a>(Cow<'a, str>);

impl<'a> Username<'a> {
    pub fn new<T>(username: T) -> Self
    where T: Into<Cow<'a, str>>
    {
        Username(username.into())
    }

    pub fn as_str(&'a self) -> &'a str {
        self.0.borrow()
    }
}

impl<'a, T> From<T> for Username<'a>
    where T: Into<Cow<'a, str>>
{
    fn from(value: T) -> Username<'a> {
        Username(value.into())
    }
}