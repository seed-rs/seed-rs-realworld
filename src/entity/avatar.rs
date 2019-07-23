use crate::entity::Image;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Avatar(Option<Cow<'static, str>>);

impl Avatar {
    pub fn new(url: Option<impl Into<Cow<'static, str>>>) -> Self {
        Self(url.map(Into::into))
    }

    pub fn src(&self) -> String {
        match &self.0 {
            Some(url) if !url.is_empty() => url.to_string(),
            _ => Image::default_avatar().into_url(),
        }
    }
}
