use serde::Deserialize;
use crate::{avatar, session, article, author, profile};
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;
use std::rc::Rc;
use std::convert::TryInto;
use article::tag::IntoTags;

#[derive(Deserialize)]
struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    article: ServerDataFields
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerDataFields {
    title: String,
    slug: String,
    body: String,
    created_at: String,
    updated_at: String,
    tag_list: Vec<String>,
    description: String,
    author: ServerDataFieldAuthor,
    favorited: bool,
    favorites_count: usize,
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
    fn try_into_article(self, session: session::Session) -> Result<article::Article, String> {
        let created_at = self.article.created_at.try_into()?;
        let updated_at = self.article.updated_at.try_into()?;

        Ok(article::Article {
            title: self.article.title,
            slug: self.article.slug.into(),
            body: self.article.body.into(),
            created_at,
            updated_at,
            tag_list: self.article.tag_list.into_tags(),
            description: self.article.description,
            author: self.article.author.into_author(session),
            favorited: self.article.favorited,
            favorites_count: self.article.favorites_count,
        })
    }
}

pub fn favorite<Ms: 'static>(
    session: &session::Session,
    slug: &article::slug::Slug,
    f: fn(Result<article::Article, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let slug = slug.clone();
    let session = session.clone();

    let mut request = fetch::Request::new(
        format!("https://conduit.productionready.io/api/articles/{}/favorite", slug.as_str())
    )
        .method(fetch::Method::Post)
        .timeout(5000);

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
) -> Result<article::Article, Vec<String>> {
    match fetch_object.result {
        Err(_) => {
            Err(vec!["Request error".into()])
        },
        Ok(response) => {
            if response.status.is_ok() {
                    let article =
                        response
                            .data
                            .and_then(|string| {
                                serde_json::from_str::<ServerData>(string.as_str())
                                    .map_err(|error| {
                                        fetch::DataError::SerdeError(Rc::new(error))
                                    })
                            })
                            .map(|server_data| {
                                server_data.try_into_article(session)
                            });

                    match article {
                        Ok(article) => {
                            match article {
                                Ok(article) => Ok(article),
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
