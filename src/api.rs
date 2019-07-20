use crate::{username, viewer};
use serde::{Deserialize, Serialize};
use seed::storage;
use serde_json;

const STORAGE_KEY: &'static str = "conduit";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub username: username::Username<'static>,
    pub auth_token: String
}

impl Credentials{
    pub fn username(&self) -> &username::Username {
        &self.username
    }
}

pub fn load_viewer() -> Option<viewer::Viewer> {
    local_storage()
        .get_item(STORAGE_KEY)
        .expect("try to get local storage item failed")
        .map(|serialized_item|{
            serde_json::from_str(&serialized_item).expect("viewer deserialization failed")
        })
}

pub fn store_viewer(viewer: &viewer::Viewer) {
    storage::store_data(&local_storage(), STORAGE_KEY, viewer);
}

pub fn logout() {
    local_storage().remove_item(STORAGE_KEY).expect("remove item from local storage failed");
}

fn local_storage() -> storage::Storage {
    storage::get_storage().expect("get local storage failed")
}