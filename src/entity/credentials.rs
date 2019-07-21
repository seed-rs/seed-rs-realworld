use serde::{Deserialize, Serialize};
use crate::entity::username;

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