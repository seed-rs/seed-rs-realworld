use serde::{Serialize, Deserialize};
use crate::{avatar, session, article, author, profile};
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;
use std::rc::Rc;
use std::convert::TryInto;

#[derive(Deserialize)]
struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    comment: ServerDataFields
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerDataFields {
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

    let mut request = fetch::Request::new(
        format!("https://conduit.productionready.io/api/articles/{}/comments", slug.as_str()).into()
    )
        .method(fetch::Method::Post)
        .timeout(5000)
        .send_json(&dto);

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
) -> Result<article::comment::Comment<'static>, Vec<String>> {
    match fetch_object.result {
        Err(_) => {
            Err(vec!["Request error".into()])
        },
        Ok(response) => {
            if response.status.is_ok() {
                    let comment =
                        response
                            .data
                            .and_then(|string| {
                                serde_json::from_str::<ServerData>(string.as_str())
                                    .map_err(|error| {
                                        fetch::DataError::SerdeError(Rc::new(error))
                                    })
                            })
                            .map(|server_data| {
                                server_data.try_into_comment(session)
                            });

                    match comment {
                        Ok(comment) => {
                            match comment {
                                Ok(comment) => Ok(comment),
                                Err(error) => {
                                    Err(vec![error])
                                }
                            }
                        },
                        Err(_) => {
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
                    Err(_) => {
                        Err(vec!["Data error".into()])
                    }
                }
            }
        }
    }
}
