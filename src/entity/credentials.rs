use serde::{Deserialize, Serialize};
use crate::entity::Username;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub username: Username<'static>,
    pub auth_token: String
}

impl Credentials{
    pub fn username(&self) -> &Username {
        &self.username
    }
}