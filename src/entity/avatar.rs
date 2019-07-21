use std::borrow::Cow;
use crate::entity::asset;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Avatar(Option<Cow<'static, str>>);

impl Avatar {
    pub fn new(url: Option<impl Into<Cow<'static, str>>>) -> Self
    {
        Avatar(url.map(Into::into))
    }

    pub fn src(&self) -> String {
        match &self.0 {
            Some(url) if !url.is_empty() => url.to_string(),
            _ => asset::default_avatar().into_url()
        }
    }
}
