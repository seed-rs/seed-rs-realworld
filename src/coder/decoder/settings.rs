use serde::Deserialize;
use crate::entity::{form::settings as form};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    email: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

impl Settings {
    pub fn into_form(self) -> form::Form {
        let fields: Vec<form::Field> = vec![
            form::Field::Avatar(self.image.unwrap_or_default()),
            form::Field::Username(self.username),
            form::Field::Bio(self.bio.unwrap_or_default()),
            form::Field::Email(self.email),
            form::Field::Password(String::default()),
        ];
        form::Form::new(fields)
    }
}