use crate::{username, profile, api};

pub enum Author<'a> {
    Following(FollowedAuthor<'a>),
    NotFollowing(UnfollowedAuthor<'a>),
    IsViewer(api::Credentials, profile::Profile),
}

pub struct FollowedAuthor<'a>(pub username::Username<'a>, pub profile::Profile);

pub struct UnfollowedAuthor<'a>(pub username::Username<'a>, pub profile::Profile);