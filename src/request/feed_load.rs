use serde::Deserialize;
use crate::{viewer, avatar, username, api, session, article, page, paginated_list, author, profile, timestamp, page_number};
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;
use std::rc::Rc;
use std::convert::TryFrom;
use std::convert::TryInto;
use article::tag::IntoTags;

const ARTICLES_PER_PAGE: usize = 5;

#[derive(Deserialize)]
struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    articles: Vec<ServerDataItemArticle>,
    articles_count: usize
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerDataItemArticle {
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
    fn into_paginated_list(self, session: session::Session) -> paginated_list::PaginatedList<article::Article> {
        paginated_list::PaginatedList {
            values: self.articles.into_iter().map(|item| {
                let created_at = match timestamp::Timestamp::try_from(item.created_at) {
                    Ok(timestamp) => timestamp,
                    Err(error) => return Err(error)
                };
                let updated_at = timestamp::Timestamp::try_from(item.updated_at)?;

                Ok(article::Article {
                    title: item.title,
                    slug: item.slug.into(),
                    body: item.body.into(),
                    created_at,
                    updated_at,
                    tag_list: item.tag_list.into_tags(),
                    description: item.description,
                    author: item.author.into_author(session.clone()),
                    favorited: item.favorited,
                    favorites_count: item.favorites_count,
                })
                // @TODO log errors?
            }).filter_map(Result::ok).collect(),
            per_page: ARTICLES_PER_PAGE,
            total: self.articles_count
        }
    }
}

pub fn request_url(
    username: &username::Username<'static>,
    feed_tab: &page::profile::FeedTab,
    page_number: page_number::PageNumber,
) -> String {
    format!(
        "https://conduit.productionready.io/api/articles?{}={}&limit={}&offset={}",
        match feed_tab {
            page::profile::FeedTab::MyArticles => "author",
            page::profile::FeedTab::FavoritedArticles => "favorited",
        },
        username.as_str(),
        ARTICLES_PER_PAGE,
        (page_number.to_usize() - 1) * ARTICLES_PER_PAGE
    )
}

pub fn load_feed<Ms: 'static>(
    session: session::Session,
    username: username::Username<'static>,
    feed_tab: page::profile::FeedTab,
    page_number: page_number::PageNumber,
    f: fn(Result<paginated_list::PaginatedList<article::Article>, (username::Username<'static>, Vec<String>)>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let session = session.clone();
    let username = username.clone();

    let mut request = fetch::Request::new(
        request_url(&username, &feed_tab, page_number)
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
) -> Result<paginated_list::PaginatedList<article::Article>, (username::Username<'static>, Vec<String>)> {
    match fetch_object.result {
        Err(request_error) => {
            Err((username, vec!["Request error".into()]))
        },
        Ok(response) => {
            if response.status.is_ok() {
                    let paginated_list =
                        response
                            .data
                            .and_then(|string| {
                                serde_json::from_str::<ServerData>(string.as_str())
                                    .map_err(|error| {
                                        fetch::DataError::SerdeError(Rc::new(error))
                                    })
                            })
                            .map(|server_data| {
                                server_data.into_paginated_list(session)
                            });

                    match paginated_list {
                        Ok(paginated_list) => {
                            Ok(paginated_list)
                        },
                        Err(data_error) => {
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
                    Err(data_error) => {
                        Err((username, vec!["Data error".into()]))
                    }
                }
            }
        }
    }
}
