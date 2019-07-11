use crate::{username, profile, api};
use seed::prelude::*;

#[derive(Clone)]
pub enum Author<'a> {
    Following(FollowedAuthor<'a>),
    NotFollowing(UnfollowedAuthor<'a>),
    IsViewer(api::Credentials, profile::Profile),
}

impl<'a> Author<'a> {
    pub fn username(&'a self) -> &'a username::Username<'a> {
        match self {
            Author::Following(FollowedAuthor(username, _)) => username,
            Author::NotFollowing(UnfollowedAuthor(username, _)) => username,
            Author::IsViewer(credentials,_) => credentials.username(),
        }
    }

    pub fn profile(&self) -> &profile::Profile {
        match self {
            Author::Following(FollowedAuthor(_, profile)) => profile,
            Author::NotFollowing(UnfollowedAuthor(_, profile)) => profile,
            Author::IsViewer(_, profile) => profile,
        }
    }
}

#[derive(Clone)]
pub struct FollowedAuthor<'a>(pub username::Username<'a>, pub profile::Profile);

impl<'a> FollowedAuthor<'a> {
    pub fn to_static(&self) -> FollowedAuthor<'static> {
        FollowedAuthor(self.0.to_string().into(), self.1.clone())
    }
}

#[derive(Clone)]
pub struct UnfollowedAuthor<'a>(pub username::Username<'a>, pub profile::Profile);

impl<'a> UnfollowedAuthor<'a> {
    pub fn to_static(&self) -> UnfollowedAuthor<'static> {
        UnfollowedAuthor(self.0.to_string().into(), self.1.clone())
    }
}

pub fn view_follow_button<Ms: Clone>(msg: Ms, username: &username::Username) -> Node<Ms> {
    button![
        class!["btn", "btn-sm", "btn-outline-secondary", "action-btn"],
        i![
            class!["ion-plus-round"]
        ],
        format!("\u{00A0}Follow {}", username.as_str()),
        simple_ev(Ev::Click, msg)
    ]
}

pub fn view_unfollow_button<Ms: Clone>(msg: Ms, username: &username::Username) -> Node<Ms> {
    button![
        class!["btn", "btn-sm", "btn-secondary", "action-btn"],
        i![
            class!["ion-plus-round"]
        ],
        format!("\u{00A0}Unfollow {}", username.as_str()),
        simple_ev(Ev::Click, msg)
    ]
}