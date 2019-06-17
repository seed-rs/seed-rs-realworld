use crate::username;

pub struct Credentials<'a> {
    username: username::Username<'a>,
    auth_token: &'a str
}

pub fn logout() {

}