use crate::entity::avatar;

#[derive(Clone)]
pub struct Profile {
    pub bio: Option<String>,
    pub avatar: avatar::Avatar
}