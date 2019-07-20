use serde::Deserialize;
use crate::{avatar, username, session, article, page, paginated_list, author, profile, timestamp, page_number, logger, request};
use futures::prelude::*;
use seed::fetch;
use std::convert::TryFrom;
use article::tag::IntoTags;

const ARTICLES_PER_PAGE: usize = 5;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    articles: Vec<ServerDataItemArticle>,
    articles_count: usize
}

#[derive(Deserialize, Debug)]
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
    fn into_paginated_list(self, session: session::Session) -> paginated_list::PaginatedList<article::Article> {
        paginated_list::PaginatedList {
            values: self.articles.into_iter().map(|item| {
                let created_at = match timestamp::Timestamp::try_from(item.created_at) {
                    Ok(timestamp) => timestamp,
                    Err(error) => {
                        logger::error(error.clone());
                        return Err(error)
                    }
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
        "articles?{}={}&limit={}&offset={}",
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

    request::new_api_request(
        &request_url(&username, &feed_tab, page_number),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<ServerData>| {
            f(data_result
                .map(move |server_data| server_data.into_paginated_list(session))
                .map_err(request::fail_reason_into_errors)
                .map_err(|problems| (username, problems))
            )
        })
}