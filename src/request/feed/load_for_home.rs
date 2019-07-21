use serde::Deserialize;
use crate::entity::{Credentials, article, paginated_list, page_number};
use crate::{page, logger, request, dto};
use futures::prelude::*;
use seed::fetch;

const ARTICLES_PER_PAGE: usize = 10;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    articles: Vec<dto::ArticleDto>,
    articles_count: usize
}

impl RootDto {
    fn into_paginated_list(self, credentials: Option<Credentials>,) -> paginated_list::PaginatedList<article::Article> {
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
    feed_tab: &page::home::FeedTab,
    page_number: page_number::PageNumber,
) -> String {
    // @TODO refactor!
    format!(
        "articles{}?{}limit={}&offset={}",
        match feed_tab {
            page::home::FeedTab::YourFeed(_) => "/feed",
            page::home::FeedTab::GlobalFeed => "",
            page::home::FeedTab::TagFeed(_) => "",
        },
        match feed_tab {
            page::home::FeedTab::YourFeed(_) => "".to_string(),
            page::home::FeedTab::GlobalFeed => "".to_string(),
            page::home::FeedTab::TagFeed(tag) => format!("tag={}&", tag),
        },
        ARTICLES_PER_PAGE,
        (page_number.to_usize() - 1) * ARTICLES_PER_PAGE
    )
}

pub fn load_for_home<Ms: 'static>(
    credentials: Option<Credentials>,
    feed_tab: &page::home::FeedTab,
    page_number: page_number::PageNumber,
    f: fn(Result<paginated_list::PaginatedList<article::Article>, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &request_url(feed_tab, page_number),
        credentials.as_ref()
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(move |root_dto| root_dto.into_paginated_list(credentials))
                .map_err(request::fail_reason_into_errors)
            )
        })
}