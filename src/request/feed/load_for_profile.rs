use serde::Deserialize;
use crate::entity::{Username, Credentials, Article, PaginatedList, PageNumber};
use crate::{page, logger, request, coder::decoder};
use futures::prelude::*;
use seed::fetch;
use std::num::NonZeroUsize;
use lazy_static::lazy_static;

lazy_static! {
    static ref ARTICLES_PER_PAGE: NonZeroUsize = NonZeroUsize::new(5).unwrap();
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    articles: Vec<decoder::Article>,
    articles_count: usize
}

impl RootDecoder {
    fn into_paginated_list(self, credentials: Option<Credentials>,) -> PaginatedList<Article> {
        PaginatedList {
            items: self.articles.into_iter().filter_map(|article_decoder| {
                match article_decoder.try_into_article(credentials.clone()) {
                    Ok(article) => Some(article),
                    Err(error) => {
                        logger::error(error);
                        None
                    }
                }
            }).collect(),
            per_page: *ARTICLES_PER_PAGE,
            total: self.articles_count
        }
    }
}

pub fn request_url(
    username: &Username<'static>,
    feed_tab: &page::profile::FeedTab,
    page_number: PageNumber,
) -> String {
    format!(
        "articles?{}={}&limit={}&offset={}",
        match feed_tab {
            page::profile::FeedTab::MyArticles => "author",
            page::profile::FeedTab::FavoritedArticles => "favorited",
        },
        username.as_str(),
        *ARTICLES_PER_PAGE,
        (page_number.to_usize() - 1) * ARTICLES_PER_PAGE.get()
    )
}

pub fn load_for_profile<Ms: 'static>(
    credentials: Option<Credentials>,
    username: Username<'static>,
    feed_tab: &page::profile::FeedTab,
    page_number: PageNumber,
    f: fn(Result<PaginatedList<Article>, (Username<'static>, Vec<String>)>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &request_url(&username, &feed_tab, page_number),
        credentials.as_ref()
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |root_decoder| root_decoder.into_paginated_list(credentials))
                .map_err(request::fail_reason_into_errors)
                .map_err(|errors| (username, errors))
            )
        })
}