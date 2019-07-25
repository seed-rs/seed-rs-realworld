use crate::entity::form::settings::{Field, Form};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    email: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

impl Settings {
    pub fn into_form(self) -> Form {
        Form::new(vec![
            Field::Avatar(self.image.unwrap_or_default()),
            Field::Username(self.username),
            Field::Bio(self.bio.unwrap_or_default()),
            Field::Email(self.email),
            Field::Password(String::default()),
        ])
    }
}

// ====== ====== TESTS ====== ======

// see `src/code/decoder/viewer` for example how to test decoder
