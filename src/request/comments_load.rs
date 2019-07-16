use serde::Deserialize;
use crate::{viewer, avatar, username, api, session, article, page, paginated_list, author, profile, timestamp, page_number, logger};
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;
use std::rc::Rc;
use std::convert::TryFrom;
use std::convert::TryInto;
use article::tag::IntoTags;
use std::collections::VecDeque;

const ARTICLES_PER_PAGE: usize = 5;

#[derive(Deserialize)]
struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    comments: VecDeque<ServerDataItemComment>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerDataItemComment {
    id: usize,
    created_at: String,
    updated_at: String,
    body: String,
    author: ServerDataFieldAuthor,
}

#[derive(Deserialize)]
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

    let mut request = fetch::Request::new(
        format!("https://conduit.productionready.io/api/articles/{}/comments", slug.as_str())
    ).timeout(5000);

    if let Some(viewer) = session.viewer() {
        let auth_token = viewer.credentials.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }

    request.fetch_string(move |fetch_object| {
        f(process_fetch_object(session, fetch_object))
    })
}

fn process_fetch_object(
    session: session::Session,
    fetch_object: fetch::FetchObject<String>
) -> Result<VecDeque<article::comment::Comment<'static>>, Vec<String>> {
    match fetch_object.result {
        Err(request_error) => {
            Err(vec!["Request error".into()])
        },
        Ok(response) => {
            if response.status.is_ok() {
                    let comments =
                        response
                            .data
                            .and_then(|string| {
                                serde_json::from_str::<ServerData>(string.as_str())
                                    .map_err(|error| {
                                        fetch::DataError::SerdeError(Rc::new(error))
                                    })
                            })
                            .map(|server_data| {
                                server_data.into_comments(session)
                            });

                    match comments {
                        Ok(comments) => {
                            Ok(comments)
                        },
                        Err(data_error) => {
                            Err(vec!["Data error".into()])
                        }
                    }
            } else {
                let error_messages: Result<Vec<String>, fetch::DataError> =
                    response
                        .data
                        .and_then(|string| {
                            serde_json::from_str::<ServerErrorData>(string.as_str())
                                .map_err(|error| {
                                    fetch::DataError::SerdeError(Rc::new(error))
                                })
                        }).and_then(|server_error_data| {
                        Ok(server_error_data.errors.into_iter().map(|(field, errors)| {
                            format!("{} {}", field, errors.join(", "))
                        }).collect())
                    });
                match error_messages {
                    Ok(error_messages) => {
                        Err(error_messages)
                    },
                    Err(data_error) => {
                        Err(vec!["Data error".into()])
                    }
                }
            }
        }
    }
}
