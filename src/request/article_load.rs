use serde::Deserialize;
use crate::{viewer, avatar, username, api, form::article_editor as form, session, article};
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
    bio: String,
    image: String,
    following: bool,
}

impl ServerData {
    fn into_article(self) -> article::Article {
        article::Article {
            title: self.article.title,
            slug: self.article.slug.into(),
            body: self.article.body,
            created_at: self.article.created_at,
            updated_at: self.article.updated_at,
            tag_list: self.article.tag_list,
            description: self.article.description,
            author: article::Author {
                username: self.article.author.username.into(),
                bio: self.article.author.bio,
                image: self.article.author.image,
                following: self.article.author.following,
            },
            favorited: self.article.favorited,
            favorites_count: self.article.favorites_count,
        }
    }
}

pub fn load_article<Ms: 'static>(
    session: &session::Session,
    slug: &article::slug::Slug,
    f: fn(Result<article::Article, (article::slug::Slug, Vec<form::Problem>)>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let auth_token =
        session
            .viewer()
            .map(|viewer|viewer.credentials.auth_token.as_str())
            .unwrap_or_default();

    let slug = slug.clone();
    fetch::Request::new(format!("https://conduit.productionready.io/api/articles/{}", slug.as_str()))
        .header("authorization", &format!("Token {}", auth_token))
        .timeout(5000)
        .fetch_string(move |fetch_object| {
            f(process_fetch_object(slug, fetch_object))
        })
}

fn process_fetch_object(
    slug: article::slug::Slug,
    fetch_object: fetch::FetchObject<String>
) -> Result<article::Article, (article::slug::Slug, Vec<form::Problem>)> {
    match fetch_object.result {
        Err(request_error) => {
            Err((slug, vec![form::Problem::new_server_error("Request error")]))
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
                                server_data.into_article()
                            });

                    match article {
                        Ok(article) => {
                            Ok(article)
                        },
                        Err(data_error) => {
                            Err((slug, vec![form::Problem::new_server_error("Data error")]))
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
                        let problems = error_messages
                            .into_iter()
                            .map(|message| {
                                form::Problem::new_server_error(message)
                            }).collect();
                        Err((slug, problems))
                    },
                    Err(data_error) => {
                        Err((slug, vec![form::Problem::new_server_error("Data error")]))
                    }
                }
            }
        }
    }
}
