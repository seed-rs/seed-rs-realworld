use serde::Deserialize;
use crate::entity::{self, Avatar, Profile};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Viewer{
    username: String,
    image: Option<String>,
    token: String,
    bio: Option<String>
}

impl Viewer {
    pub fn into_viewer(self) -> entity::Viewer {
        entity::Viewer {
            profile: Profile {
                avatar: Avatar::new(self.image),
                username: self.username.into(),
                bio: self.bio,
            },
            auth_token: self.token
        }
    }
}