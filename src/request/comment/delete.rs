use crate::{api, article, comment_id, request};
use futures::prelude::*;
use seed::fetch;
use indexmap::IndexMap;

type RootDto = IndexMap<(), ()>;

pub fn delete<Ms: 'static>(
    credentials: Option<&api::Credentials>,
    slug: &article::slug::Slug,
    comment_id: comment_id::CommentId,
    f: fn(Result<comment_id::CommentId, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("articles/{}/comments/{}", slug.as_str(), comment_id.as_str()),
        credentials
    )
        .method(fetch::Method::Delete)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(move |_| comment_id)
                .map_err(request::fail_reason_into_errors)
            )
        })
}