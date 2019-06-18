use crate::{avatar, api, username};

pub struct Viewer<'a> {
    avatar: avatar::Avatar<'a>,
    credentials: api::Credentials<'a>
}

impl<'a> Viewer<'a> {
    pub fn username(&self) -> &username::Username {
        self.credentials.username()
    }

    pub fn avatar(&self) -> &avatar::Avatar {
        &self.avatar
    }
}