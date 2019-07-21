use serde::Deserialize;
use crate::{session, article, request, dto};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    article: dto::article::ArticleDTO
}

pub fn load_article<Ms: 'static>(
    session: &session::Session,
    slug: &article::slug::Slug,
    f: fn(Result<article::Article, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let slug = slug.clone();
    let session = session.clone();

    request::new_api_request(
        &format!("articles/{}", slug.as_str()),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map_err(request::fail_reason_into_errors)
                .and_then(move |root_dto| {
                    root_dto.article.try_into_article(session)
                        .map_err(|error| vec![error])
                })
            )
        })
}