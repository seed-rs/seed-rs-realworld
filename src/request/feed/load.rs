use serde::Deserialize;
use crate::{username, api, article, page, paginated_list, page_number, logger, request, dto};
use futures::prelude::*;
use seed::fetch;

const ARTICLES_PER_PAGE: usize = 5;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    articles: Vec<dto::article::ArticleDTO>,
    articles_count: usize
}

impl RootDto {
    fn into_paginated_list(self, credentials: Option<api::Credentials>,) -> paginated_list::PaginatedList<article::Article> {
        paginated_list::PaginatedList {
            values: self.articles.into_iter().filter_map(|article_dto| {
                // @TODO without clone / more effective?
                match article_dto.try_into_article(credentials.clone()) {
                    Ok(article) => Some(article),
                    Err(error) => {
                        logger::error(error);
                        None
                    }
                }
            }).collect(),
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

pub fn load<Ms: 'static>(
    credentials: Option<api::Credentials>,
    username: username::Username<'static>,
    feed_tab: &page::profile::FeedTab,
    page_number: page_number::PageNumber,
    f: fn(Result<paginated_list::PaginatedList<article::Article>, (username::Username<'static>, Vec<String>)>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &request_url(&username, &feed_tab, page_number),
        credentials.as_ref()
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(move |root_dto| root_dto.into_paginated_list(credentials))
                .map_err(request::fail_reason_into_errors)
                .map_err(|errors| (username, errors))
            )
        })
}