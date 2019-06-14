use std::borrow::Cow;
use crate::asset;

pub struct Avatar<'a>(Cow<'a, str>);

impl<'a> Avatar<'a> {
    pub fn new<T>(url: Option<T>) -> Self
        where T: Into<Cow<'a, str>>
    {
        match url {
            Some(url) => Avatar(url.into()),
            None => Avatar(asset::default_avatar().into_url().into())
        }
    }

    pub fn src(&self) -> &str {
        &self.0
    }
}
