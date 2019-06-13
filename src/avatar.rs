use crate::asset;

#[derive(Clone)]
pub struct Avatar(String);

impl Avatar {
    pub fn new(url: Option<String>) -> Self {
        Avatar(url.unwrap_or_else(|| asset::default_avatar().url().into()))
    }

    pub fn src(&self) -> &str {
        &self.0
    }
}
