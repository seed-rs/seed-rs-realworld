use crate::username;

pub struct Credentials<'a> {
    username: username::Username<'a>,
    auth_token: &'a str
}

impl<'a> Credentials<'a>{
    pub fn username(&self) -> &username::Username {
        &self.username
    }
}

pub fn logout() {

}