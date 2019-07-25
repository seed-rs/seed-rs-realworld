use crate::entity::{self, Avatar, Profile};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    username: String,
    image: Option<String>,
    token: String,
    bio: Option<String>,
}

impl Viewer {
    pub fn into_viewer(self) -> entity::Viewer {
        entity::Viewer {
            profile: Profile {
                avatar: Avatar::new(self.image),
                username: self.username.into(),
                bio: self.bio,
            },
            auth_token: self.token,
        }
    }
}

// ====== ====== TESTS ====== ======

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json::{self, json};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn decode_viewer_test() {
        // ====== ARRANGE ======
        let json = json!({
            "username": "John",
            "image": null,
            "token": "John's token",
        });

        // ====== ACT ======
        let viewer = serde_json::from_value::<Viewer>(json)
            .expect("deserialize Viewer failed")
            .into_viewer();

        // ====== ASSERT ======
        assert_eq!(viewer.profile.username.as_str(), "John");
    }
}
