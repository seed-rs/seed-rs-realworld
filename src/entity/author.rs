use crate::entity::{Profile, Username, Viewer};
use crate::Route;
use seed::prelude::*;
use std::borrow::Cow;

#[derive(Clone)]
pub enum Author {
    Following(FollowedAuthor),
    NotFollowing(UnfollowedAuthor),
    IsViewer(Viewer),
}

impl Author {
    pub fn username(&self) -> &Username {
        match self {
            Author::Following(FollowedAuthor { profile })
            | Author::NotFollowing(UnfollowedAuthor { profile }) => &profile.username,
            Author::IsViewer(viewer) => viewer.username(),
        }
    }

    pub fn profile(&self) -> &Profile {
        match self {
            Author::Following(FollowedAuthor { profile, .. })
            | Author::NotFollowing(UnfollowedAuthor { profile, .. }) => profile,
            Author::IsViewer(viewer) => viewer.profile(),
        }
    }
}

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FollowedAuthor {
    pub profile: Profile,
}

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct UnfollowedAuthor {
    pub profile: Profile,
}

pub fn view<Ms>(username: &Username) -> Node<Ms> {
    a![
        class!["author"],
        attrs! {At::Href => Route::Profile(Cow::Borrowed(username)).to_string()},
        username.to_string()
    ]
}

pub fn view_follow_button<Ms: Clone>(msg: Ms, username: &Username) -> Node<Ms> {
    button![
        class!["btn", "btn-sm", "btn-outline-secondary", "action-btn"],
        i![class!["ion-plus-round"]],
        format!("\u{00A0}Follow {}", username.as_str()),
        simple_ev(Ev::Click, msg)
    ]
}

pub fn view_unfollow_button<Ms: Clone>(msg: Ms, username: &Username) -> Node<Ms> {
    button![
        class!["btn", "btn-sm", "btn-secondary", "action-btn"],
        i![class!["ion-plus-round"]],
        format!("\u{00A0}Unfollow {}", username.as_str()),
        simple_ev(Ev::Click, msg)
    ]
}
