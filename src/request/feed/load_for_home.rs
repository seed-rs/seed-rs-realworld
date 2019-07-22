use crate::entity::{Article, PageNumber, PaginatedList, Viewer};
use crate::{coder::decoder, logger, page, request};
use futures::prelude::*;
use lazy_static::lazy_static;
use seed::fetch;
use serde::Deserialize;
use std::borrow::Cow;
use std::num::NonZeroUsize;

lazy_static! {
    static ref ARTICLES_PER_PAGE: NonZeroUsize = NonZeroUsize::new(10).unwrap();
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    articles: Vec<decoder::Article>,
    articles_count: usize,
}

impl RootDecoder {
    fn into_paginated_list(self, viewer: Option<Viewer>) -> PaginatedList<Article> {
        PaginatedList {
            items: self
                .articles
                .into_iter()
                .filter_map(|article_decoder| {
                    match article_decoder.try_into_article(viewer.as_ref().map(Cow::Borrowed)) {
                        Ok(article) => Some(article),
                        Err(error) => {
                            logger::error(error);
                            None
                        }
                    }
                })
                .collect(),
            per_page: *ARTICLES_PER_PAGE,
            total: self.articles_count,
        }
    }
}

pub fn request_url(feed_tab: &page::home::FeedTab, page_number: PageNumber) -> String {
    use page::home::FeedTab::*;

    let (path, tag_param) = match feed_tab {
        YourFeed(_) => (Some("/feed"), None),
        GlobalFeed => (None, None),
        TagFeed(tag) => (None, Some(format!("tag={}", tag))),
    };

    let mut parameters = vec![
        format!("limit={}", *ARTICLES_PER_PAGE),
        format!(
            "offset={}",
            (page_number.to_usize() - 1) * ARTICLES_PER_PAGE.get()
        ),
    ];
    if let Some(tag_param) = tag_param {
        parameters.push(tag_param)
    }
    format!(
        "articles{}?{}",
        path.unwrap_or_default(),
        parameters.join("&")
    )
}

pub fn load_for_home<Ms: 'static>(
    viewer: Option<Viewer>,
    feed_tab: &page::home::FeedTab,
    page_number: PageNumber,
    f: fn(Result<PaginatedList<Article>, Vec<String>>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new_api_request(&request_url(feed_tab, page_number), viewer.as_ref()).fetch_json_data(
        move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |root_decoder| root_decoder.into_paginated_list(viewer))
                .map_err(request::fail_reason_into_errors))
        },
    )
}
