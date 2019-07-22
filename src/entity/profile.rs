use crate::entity::Avatar;

#[derive(Clone)]
pub struct Profile {
    pub bio: Option<String>,
    pub avatar: Avatar
}