use crate::entity::{Avatar, Credentials, Username};
use crate::storage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Viewer {
    pub avatar: Avatar,
    pub credentials: Credentials
}

impl Viewer {
    pub fn username(&self) -> &Username {
        self.credentials.username()
    }

    pub fn avatar(&self) -> &Avatar {
        &self.avatar
    }

    pub fn store(&self) {
        storage::store_viewer(self);
    }
}