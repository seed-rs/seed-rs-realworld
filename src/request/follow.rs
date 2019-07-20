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

pub fn follow<Ms: 'static>(
    session: session::Session,
    username: username::Username<'static>,
    f: fn(Result<author::Author<'static>, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let username = username.clone();
    let session = session.clone();

    let mut request = fetch::Request::new(
        format!("https://conduit.productionready.io/api/profiles/{}/follow", username.as_str())
    )
        .method(fetch::Method::Post)
        .timeout(5000);

    if let Some(viewer) = session.viewer() {
        let auth_token = viewer.credentials.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }

    request.fetch_json_data(move |data_result: fetch::ResponseDataResult<ServerData>| {
        f(data_result
            .map(move |server_data| server_data.into_author(session))
            .map_err(request::fail_reason_into_errors)
        )
    })
}