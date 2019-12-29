use crate::{
    entity::{Profile, Username, Viewer},
    Route,
};
use seed::prelude::*;
use std::borrow::Cow;

// ------ Author ------

#[derive(Clone)]
pub enum Author {
    Following(Profile),
    NotFollowing(Profile),
    IsViewer(Viewer),
}

impl Author {
    pub fn username(&self) -> &Username {
        match self {
            Self::Following(profile) | Self::NotFollowing(profile) => &profile.username,
            Self::IsViewer(viewer) => viewer.username(),
        }
    }

    pub fn profile(&self) -> &Profile {
        match self {
            Self::Following(profile) | Self::NotFollowing(profile) => profile,
            Self::IsViewer(viewer) => &viewer.profile,
        }
    }
}

// ------ view functions ------

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
