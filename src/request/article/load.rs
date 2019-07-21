use serde::Deserialize;
use crate::{api, article, request, dto};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    article: dto::article::ArticleDTO
}

pub fn load<Ms: 'static>(
    credentials: Option<api::Credentials>,
    slug: &article::slug::Slug,
    f: fn(Result<article::Article, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("articles/{}", slug.as_str()),
        credentials.as_ref()
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map_err(request::fail_reason_into_errors)
                .and_then(move |root_dto| {
                    root_dto.article.try_into_article(credentials)
                        .map_err(|error| vec![error])
                })
            )
        })
}