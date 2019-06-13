use crate::{avatar, api};

#[derive(Clone)]
pub struct Viewer {
    avatar: avatar::Avatar,
    credentials: api::Credentials
}