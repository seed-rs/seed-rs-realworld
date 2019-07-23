use crate::entity::{Article, ErrorMessage, PageNumber, PaginatedList, Username, Viewer};
use crate::{coder::decoder, logger, page::profile::SelectedFeed, request};
use futures::prelude::*;
use lazy_static::lazy_static;
use seed::fetch::ResponseDataResult;
use serde::Deserialize;
use std::borrow::Cow;
use std::num::NonZeroUsize;

lazy_static! {
    static ref ARTICLES_PER_PAGE: NonZeroUsize = NonZeroUsize::new(5).unwrap();
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    articles: Vec<decoder::Article>,
    articles_count: usize,
}

impl RootDecoder {
    fn into_paginated_list(self, viewer: Option<&Viewer>) -> PaginatedList<Article> {
        PaginatedList {
            items: self
                .articles
                .into_iter()
                .filter_map(|article_decoder| {
                    match article_decoder.try_into_article(viewer.map(Cow::Borrowed)) {
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

pub fn request_url(
    username: &Username<'static>,
    selected_feed: SelectedFeed,
    page_number: PageNumber,
) -> String {
    format!(
        "articles?{}={}&limit={}&offset={}",
        match selected_feed {
            SelectedFeed::MyArticles => "author",
            SelectedFeed::FavoritedArticles => "favorited",
        },
        username.as_str(),
        *ARTICLES_PER_PAGE,
        (*page_number - 1) * ARTICLES_PER_PAGE.get()
    )
}

#[allow(clippy::type_complexity)]
pub fn load_for_profile<Ms: 'static>(
    viewer: Option<Viewer>,
    username: Username<'static>,
    selected_feed: SelectedFeed,
    page_number: PageNumber,
    f: fn(Result<PaginatedList<Article>, (Username<'static>, Vec<ErrorMessage>)>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new(
        &request_url(&username, selected_feed, page_number),
        viewer.as_ref(),
    )
    .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
        f(data_result
            .map(move |root_decoder| root_decoder.into_paginated_list(viewer.as_ref()))
            .map_err(request::fail_reason_into_errors)
            .map_err(|errors| (username, errors)))
    })
}
