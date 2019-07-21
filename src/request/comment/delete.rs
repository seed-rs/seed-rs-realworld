use crate::entity::{Credentials, article};
use crate::request;
use futures::prelude::*;
use seed::fetch;
use indexmap::IndexMap;

type RootDecoder = IndexMap<(), ()>;

pub fn delete<Ms: 'static>(
    credentials: Option<&Credentials>,
    slug: &article::slug::Slug,
    comment_id: article::comment::CommentId,
    f: fn(Result<article::comment::CommentId, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("articles/{}/comments/{}", slug.as_str(), comment_id.as_str()),
        credentials
    )
        .method(fetch::Method::Delete)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |_| comment_id)
                .map_err(request::fail_reason_into_errors)
            )
        })
}