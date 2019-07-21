use serde::Deserialize;
use crate::{avatar, profile, author, api};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthorDTO {
    username: String,
    bio: Option<String>,
    image: String,
    following: bool,
}

impl AuthorDTO {
    pub fn into_author(self, credentials: Option<api::Credentials>) -> author::Author<'static> {
        let username = self.username.into();
        let profile = profile::Profile {
            bio: self.bio,
            avatar: avatar::Avatar::new(Some(self.image)),
        };

        if let Some(credentials) = credentials {
            if &username == credentials.username() {
                return author::Author::IsViewer(credentials, profile)
            }
        }

        if self.following {
            author::Author::Following(
                author::FollowedAuthor(username, profile)
            )
        } else {
            author::Author::NotFollowing(
                author::UnfollowedAuthor(username, profile)
            )
        }
    }
}