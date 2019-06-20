use std::borrow::Cow;
use crate::asset;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Avatar(Cow<'static, str>);

impl Avatar {
    pub fn new<T>(url: Option<T>) -> Self
        where T: Into<Cow<'static, str>>
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
