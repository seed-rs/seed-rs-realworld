use serde::{Serialize, Deserialize};
use crate::{avatar, session, article, author, profile, request};
use futures::prelude::*;
use seed::fetch;
use std::convert::TryInto;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    comment: ServerDataFields
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerDataFields {
    id: usize,
    created_at: String,
    updated_at: String,
    body: String,
    author: ServerDataFieldAuthor,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerDataFieldAuthor {
    username: String,
    bio: Option<String>,
    image: String,
    following: bool,
}

impl ServerDataFieldAuthor {
    fn into_author(self, session: session::Session) -> author::Author<'static> {
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

impl ServerData {
    fn try_into_comment<'a>(self, session: session::Session) -> Result<article::comment::Comment<'a>, String> {
        let created_at = self.comment.created_at.try_into()?;
        let updated_at = self.comment.updated_at.try_into()?;

        Ok(article::comment::Comment {
            id: self.comment.id.to_string().into(),
            body: self.comment.body,
            created_at,
            updated_at,
            author: self.comment.author.into_author(session),
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommentToSendDTOFields {
    body: String
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommentToSendDTO {
    comment: CommentToSendDTOFields
}

pub fn create_comment<Ms: 'static>(
    session: &session::Session,
    slug: &article::slug::Slug,
    text: String,
    f: fn(Result<article::comment::Comment<'static>, Vec<String>>) -> Ms
) -> impl Future<Item=Ms, Error=Ms>  {
    let session = session.clone();

    let dto = CommentToSendDTO {
        comment: CommentToSendDTOFields {
            body: text
        }
    };

    request::new_api_request(
        &format!("articles/{}/comments", slug.as_str()),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .method(fetch::Method::Post)
        .send_json(&dto)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<ServerData>| {
            f(data_result
                .map_err(request::fail_reason_into_errors)
                .and_then(move |server_data| {
                    server_data.try_into_comment(session)
                        .map_err(|error| vec![error])
                })
            )
        })
}