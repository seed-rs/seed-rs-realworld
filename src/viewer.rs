use crate::{avatar, api, username};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Viewer {
    pub avatar: avatar::Avatar,
    pub credentials: api::Credentials
}

impl Viewer {
    pub fn username(&self) -> &username::Username {
        self.credentials.username()
    }

    pub fn avatar(&self) -> &avatar::Avatar {
        &self.avatar
    }

    pub fn store(&self) {
        api::store_viewer(self);
    }
}