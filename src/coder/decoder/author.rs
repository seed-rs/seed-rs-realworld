use serde::Deserialize;
use crate::entity::{self, Avatar, Profile, FollowedAuthor, UnfollowedAuthor, Credentials};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    username: String,
    bio: Option<String>,
    image: String,
    following: bool,
}

impl Author {
    pub fn into_author(self, credentials: Option<Credentials>) -> entity::Author<'static> {
        let username = self.username.into();
        let profile = Profile {
            bio: self.bio,
            avatar: Avatar::new(Some(self.image)),
        };

        if let Some(credentials) = credentials {
            if &username == credentials.username() {
                return entity::Author::IsViewer(credentials, profile)
            }
        }

        if self.following {
            entity::Author::Following(
                FollowedAuthor { username, profile }
            )
        } else {
            entity::Author::NotFollowing(
                UnfollowedAuthor { username, profile }
            )
        }
    }
}