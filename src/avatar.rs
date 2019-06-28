use std::borrow::Cow;
use crate::asset;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Avatar(Cow<'static, str>);

impl Avatar {
    pub fn new(url: Option<impl Into<Cow<'static, str>>>) -> Self
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
