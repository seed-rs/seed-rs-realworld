use crate::{avatar, api, username};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Viewer {
    avatar: avatar::Avatar,
    credentials: api::Credentials
}

impl Viewer {
    pub fn username(&self) -> &username::Username {
        self.credentials.username()
    }

    pub fn avatar(&self) -> &avatar::Avatar {
        &self.avatar
    }
}