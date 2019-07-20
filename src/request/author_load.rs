use serde::Deserialize;
use crate::{username, session, author, profile, avatar};
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;
use std::rc::Rc;

#[derive(Deserialize)]
struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    profile: ServerDataFields
}

#[derive(Deserialize)]
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

pub fn load_author<Ms: 'static>(
    session: session::Session,
    username: username::Username<'static>,
    f: fn(Result<author::Author<'static>, (username::Username<'static>, Vec<String>)>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let username = username.clone();
    let session = session.clone();

    let mut request = fetch::Request::new(
        format!("https://conduit.productionready.io/api/profiles/{}", username.as_str())
    ).timeout(5000);

    if let Some(viewer) = session.viewer() {
        let auth_token = viewer.credentials.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }

    request.fetch_string(move |fetch_object| {
        f(process_fetch_object(session, username, fetch_object))
    })
}

fn process_fetch_object(
    session: session::Session,
    username: username::Username<'static>,
    fetch_object: fetch::FetchObject<String>
) -> Result<author::Author<'static>, (username::Username<'static>, Vec<String>)> {
    match fetch_object.result {
        Err(_) => {
            Err((username, vec!["Request error".into()]))
        },
        Ok(response) => {
            if response.status.is_ok() {
                    let author =
                        response
                            .data
                            .and_then(|string| {
                                serde_json::from_str::<ServerData>(string.as_str())
                                    .map_err(|error| {
                                        fetch::DataError::SerdeError(Rc::new(error))
                                    })
                            })
                            .map(|server_data| {
                                server_data.into_author(session)
                            });

                    match author {
                        Ok(author) => {
                            Ok(author)
                        },
                        Err(_) => {
                            Err((username, vec!["Data error".into()]))
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
                        Err((username, error_messages))
                    },
                    Err(_) => {
                        Err((username, vec!["Data error".into()]))
                    }
                }
            }
        }
    }
}
