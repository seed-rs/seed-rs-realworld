use crate::{avatar, api};

pub struct Viewer<'a> {
    avatar: avatar::Avatar<'a>,
    credentials: api::Credentials<'a>
}