use serde::{Deserialize, Serialize};
use crate::entity::{Avatar, Username};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Profile {
    pub bio: Option<String>,
    pub avatar: Avatar,
    pub username: Username<'static>,
}