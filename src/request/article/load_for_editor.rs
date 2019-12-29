use std::{borrow::Cow, future::Future};

use seed::fetch::ResponseDataResult;
use serde::Deserialize;

use crate::{
    coder::decoder,
    entity::{form::article_editor::Problem, Article, Slug, Viewer},
    request,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    article: decoder::Article,
}

#[allow(clippy::type_complexity)]
pub fn load_for_editor<Ms: 'static>(
    viewer: Option<Viewer>,
    slug: Slug,
    f: fn(Result<Article, (Slug, Vec<Problem>)>) -> Ms,
) -> impl Future<Output = Result<Ms, Ms>> {
    request::new(&format!("articles/{}", slug.as_str()), viewer.as_ref()).fetch_json_data(
        move |data_result: ResponseDataResult<RootDecoder>| {
            f(data_result
                .map_err(request::fail_reason_into_problems)
                .and_then(move |root_decoder| {
                    root_decoder
                        .article
                        .try_into_article(viewer.map(Cow::Owned))
                        .map_err(|error| vec![Problem::new_server_error(error.into_inner())])
                })
                .map_err(|problems| (slug, problems)))
        },
    )
}
