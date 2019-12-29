use crate::{
    entity::{Avatar, Profile, Username},
    storage,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Viewer {
    pub profile: Profile,
    pub auth_token: String,
}

impl Viewer {
    pub const fn username(&self) -> &Username {
        &self.profile.username
    }

    pub const fn avatar(&self) -> &Avatar {
        &self.profile.avatar
    }

    pub fn store(&self) {
        storage::store_viewer(self);
    }
}
