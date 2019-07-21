use crate::entity::{Credentials, article};
use crate::request;
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;

type RootDecoder = IndexMap<(), ()>;

pub fn delete<Ms: 'static>(
    credentials: Option<&Credentials>,
    slug: &article::slug::Slug,
    f: fn(Result<(), Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("articles/{}", slug.as_str()),
        credentials
    )
        .method(fetch::Method::Delete)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |_| ())
                .map_err(request::fail_reason_into_errors)
            )
        })
}