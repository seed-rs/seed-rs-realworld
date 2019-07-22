use crate::entity::{Username, Credentials, Avatar};
use crate::Route;
use seed::prelude::*;
use std::borrow::Cow;

#[derive(Clone)]
pub enum Author<'a> {
    Following(FollowedAuthor<'a>),
    NotFollowing(UnfollowedAuthor<'a>),
    IsViewer(Credentials, Profile),
}

impl<'a> Author<'a> {
    pub fn username(&'a self) -> &'a Username<'a> {
        match self {
            Author::Following(FollowedAuthor { username, .. }) => username,
            Author::NotFollowing(UnfollowedAuthor { username, ..}) => username,
            Author::IsViewer(credentials,_) => credentials.username(),
        }
    }

    pub fn profile(&self) -> &Profile {
        match self {
            Author::Following(FollowedAuthor{ profile, ..}) => profile,
            Author::NotFollowing(UnfollowedAuthor{ profile, ..}) => profile,
            Author::IsViewer(_, profile) => profile,
        }
    }
}

#[derive(Clone)]
pub struct FollowedAuthor<'a> {
    pub username: Username<'a>,
    pub profile: Profile
}

#[derive(Clone)]
pub struct UnfollowedAuthor<'a> {
    pub username: Username<'a>,
    pub profile: Profile
}

#[derive(Clone)]
pub struct Profile {
    pub bio: Option<String>,
    pub avatar: Avatar
}

pub fn view<Ms>(username: &Username) -> Node<Ms> {
    a![
        class!["author"],
        attrs!{At::Href => Route::Profile(Cow::Borrowed(username)).to_string()},
        username.to_string()
    ]
}

pub fn view_follow_button<Ms: Clone>(msg: Ms, username: &Username) -> Node<Ms> {
    button![
        class!["btn", "btn-sm", "btn-outline-secondary", "action-btn"],
        i![
            class!["ion-plus-round"]
        ],
        format!("\u{00A0}Follow {}", username.as_str()),
        simple_ev(Ev::Click, msg)
    ]
}

pub fn view_unfollow_button<Ms: Clone>(msg: Ms, username: &Username) -> Node<Ms> {
    button![
        class!["btn", "btn-sm", "btn-secondary", "action-btn"],
        i![
            class!["ion-plus-round"]
        ],
        format!("\u{00A0}Unfollow {}", username.as_str()),
        simple_ev(Ev::Click, msg)
    ]
}