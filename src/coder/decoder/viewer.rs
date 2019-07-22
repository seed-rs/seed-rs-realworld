use serde::Deserialize;
use crate::entity::{self, Avatar};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Viewer{
    username: String,
    image: Option<String>,
    token: String,
}

impl Viewer {
    pub fn into_viewer(self) -> entity::Viewer {
        entity::Viewer {
            avatar: Avatar::new(self.image),
            username: self.username.into(),
            auth_token: self.token
        }
    }
}