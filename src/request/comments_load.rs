use serde::Deserialize;
use crate::{avatar, session, article, author, profile, timestamp, logger, request};
use futures::prelude::*;
use seed::fetch;
use std::convert::TryFrom;
use std::collections::VecDeque;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    comments: VecDeque<ServerDataItemComment>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerDataItemComment {
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
    fn into_comments<'a>(self, session: session::Session) -> VecDeque<article::comment::Comment<'a>> {
        self.comments.into_iter().map(|item| {
            let created_at = match timestamp::Timestamp::try_from(item.created_at) {
                Ok(timestamp) => timestamp,
                Err(error) => {
                    logger::error(error.clone());
                    return Err(error)
                }
            };
            let updated_at = timestamp::Timestamp::try_from(item.updated_at)?;

            Ok(article::comment::Comment {
                id: item.id.to_string().into(),
                body: item.body.into(),
                created_at,
                updated_at,
                author: item.author.into_author(session.clone()),
            })
        }).filter_map(Result::ok).collect()
    }
}

pub fn load_comments<Ms: 'static>(
    session: session::Session,
    slug: &article::slug::Slug,
    f: fn(Result<VecDeque<article::comment::Comment<'static>>, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let session = session.clone();

    request::new_api_request(
        &format!("articles/{}/comments", slug.as_str()),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<ServerData>| {
            f(data_result
                .map(move |server_data| server_data.into_comments(session))
                .map_err(request::fail_reason_into_errors)
            )
        })
}