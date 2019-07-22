use serde::Deserialize;
use crate::entity::{self, Avatar, Profile, FollowedAuthor, UnfollowedAuthor, Viewer};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    username: String,
    bio: Option<String>,
    image: String,
    following: bool,
}

impl Author {
    pub fn into_author(self, viewer: Option<Viewer>) -> entity::Author {
        let username = self.username.into();

        if let Some(viewer) = viewer {
            if &username == viewer.username() {
                return entity::Author::IsViewer(viewer)
            }
        }

        let profile = Profile {
            bio: self.bio,
            avatar: Avatar::new(Some(self.image)),
            username
        };

        if self.following {
            entity::Author::Following(
                FollowedAuthor { profile }
            )
        } else {
            entity::Author::NotFollowing(
                UnfollowedAuthor { profile }
            )
        }
    }
}