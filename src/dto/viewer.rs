use serde::Deserialize;
use crate::{viewer, avatar, api};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    username: String,
    image: Option<String>,
    token: String,
}

impl Viewer {
    pub fn into_viewer(self) -> viewer::Viewer {
        viewer::Viewer {
            avatar: avatar::Avatar::new(self.image),
            credentials: api::Credentials {
                username: self.username.into(),
                auth_token: self.token
            }
        }
    }
}