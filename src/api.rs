use crate::username;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Credentials {
    username: username::Username<'static>,
    auth_token: String
}

impl Credentials{
    pub fn username(&self) -> &username::Username {
        &self.username
    }
}

pub fn logout() {

}