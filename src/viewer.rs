use crate::{avatar, api};

pub struct Viewer {
    avatar: avatar::Avatar,
    credentials: api::Credentials
}