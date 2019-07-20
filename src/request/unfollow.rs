use serde::Deserialize;
use crate::{username, session, author, profile, avatar, request};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    profile: ServerDataFields
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerDataFields {
    username: String,
    bio: Option<String>,
    image: String,
    following: bool,
}

impl ServerData {
    fn into_author(self, session: session::Session) -> author::Author<'static> {
        let username = self.profile.username.into();
        let profile = profile::Profile {
            bio: self.profile.bio,
            avatar: avatar::Avatar::new(Some(self.profile.image)),
        };

        if let Some(viewer) = session.viewer() {
            if &username == viewer.username() {
                return author::Author::IsViewer(viewer.credentials.clone(), profile)
            }
        }

        if self.profile.following {
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

pub fn unfollow<Ms: 'static>(
    session: session::Session,
    username: username::Username<'static>,
    f: fn(Result<author::Author<'static>, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let username = username.clone();
    let session = session.clone();

    request::new_api_request(
        &format!("profiles/{}/follow", username.as_str()),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .method(fetch::Method::Delete)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<ServerData>| {
            f(data_result
                .map(move |server_data| server_data.into_author(session))
                .map_err(request::fail_reason_into_errors)
            )
        })
}