use crate::username;

#[derive(Clone)]
pub struct Credentials {
    username: username::Username,
    auth_token: String
}