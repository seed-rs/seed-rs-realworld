use serde::Deserialize;
use crate::{session, avatar, profile, author};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthorDTO {
    username: String,
    bio: Option<String>,
    image: String,
    following: bool,
}

impl AuthorDTO {
    pub fn into_author(self, session: session::Session) -> author::Author<'static> {
        let username = self.username.into();
        let profile = profile::Profile {
            bio: self.bio,
            avatar: avatar::Avatar::new(Some(self.image)),
        };

        if let Some(viewer) = session.viewer() {
            if &username == viewer.username() {
                return author::Author::IsViewer(viewer.credentials.clone(), profile)
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