use crate::entity::{Avatar, Username};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Profile {
    pub bio: Option<String>,
    pub avatar: Avatar,
    pub username: Username<'static>,
}
