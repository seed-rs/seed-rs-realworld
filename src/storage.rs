use crate::entity::Viewer;
use seed::storage;
use serde_json;

const STORAGE_KEY: &str = "conduit";

pub fn load_viewer() -> Option<Viewer> {
    local_storage()
        .get_item(STORAGE_KEY)
        .expect("try to get local storage item failed")
        .map(|serialized_item| {
            serde_json::from_str(&serialized_item).expect("viewer deserialization failed")
        })
}

pub fn store_viewer(viewer: &Viewer) {
    storage::store_data(&local_storage(), STORAGE_KEY, viewer);
}

pub fn delete_app_data() {
    local_storage()
        .remove_item(STORAGE_KEY)
        .expect("remove item from local storage failed");
}

// ====== PRIVATE ======

fn local_storage() -> storage::Storage {
    storage::get_storage().expect("get local storage failed")
}

// ====== ====== TESTS ====== ======

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::entity::{Avatar, Profile, Username};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn clean_local_storage() {
        local_storage().clear().expect("clear storage failed");
    }

    #[wasm_bindgen_test]
    fn load_viewer_none_test() {
        // ====== ARRANGE ======
        clean_local_storage();

        // ====== ACT & ASSERT ======
        assert!(load_viewer().is_none())
    }

    #[wasm_bindgen_test]
    fn store_view_test() {
        // ====== ARRANGE ======
        clean_local_storage();

        let viewer = Viewer {
            profile: Profile {
                username: Username::default(),
                avatar: Avatar::new(None as Option<&str>),
                bio: None,
            },
            auth_token: String::new(),
        };

        // ====== ACT ======
        store_viewer(&viewer);

        //====== ASSERT ======
        assert!(load_viewer().is_some());
    }

    #[wasm_bindgen_test]
    fn delete_app_data_test() {
        // ====== ARRANGE ======
        clean_local_storage();

        let viewer = Viewer {
            profile: Profile {
                username: Username::default(),
                avatar: Avatar::new(None as Option<&str>),
                bio: None,
            },
            auth_token: String::new(),
        };
        store_viewer(&viewer);

        // ====== ACT ======
        delete_app_data();

        // ====== ASSERT ======
        assert!(load_viewer().is_none());
    }
}
