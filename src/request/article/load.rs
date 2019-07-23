use crate::entity::{Article, ErrorMessage, Slug, Viewer};
use crate::{coder::decoder, request};
use futures::prelude::*;
use seed::fetch::ResponseDataResult;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    article: decoder::Article,
}

pub fn load<Ms: 'static>(
    viewer: Option<Viewer>,
    slug: &Slug,
    f: fn(Result<Article, Vec<ErrorMessage>>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new(&format!("articles/{}", slug.as_str()), viewer.as_ref()).fetch_json_data(
        move |data_result: ResponseDataResult<RootDecoder>| {
            f(data_result
                .map_err(request::fail_reason_into_errors)
                .and_then(move |root_decoder| {
                    root_decoder
                        .article
                        .try_into_article(viewer.map(Cow::Owned))
                        .map_err(|error| vec![error])
                }))
        },
    )
}
