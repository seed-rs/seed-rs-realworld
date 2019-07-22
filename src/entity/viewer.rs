use crate::entity::{Avatar, Username};
use crate::storage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Viewer {
    pub avatar: Avatar,
    pub username: Username<'static>,
    pub auth_token: String
}

impl Viewer {
    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn avatar(&self) -> &Avatar {
        &self.avatar
    }

    pub fn store(&self) {
        storage::store_viewer(self);
    }
}