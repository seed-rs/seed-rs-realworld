use serde::Deserialize;
use crate::entity::{form::article_editor as form, Credentials, article};
use crate::{request, coder::decoder};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    article: decoder::Article
}

pub fn load_for_editor<Ms: 'static>(
    credentials: Option<Credentials>,
    slug: article::slug::Slug,
    f: fn(Result<article::Article, (article::slug::Slug, Vec<form::Problem>)>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("articles/{}", slug.as_str()),
        credentials.as_ref()
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map_err(request::fail_reason_into_problems)
                .and_then(move |root_decoder| {
                    root_decoder.article.try_into_article(credentials)
                        .map_err(|error| vec![form::Problem::new_server_error(error)])
                })
                .map_err(|problems| (slug, problems))
            )
        })
}

