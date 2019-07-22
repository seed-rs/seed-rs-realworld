use crate::entity::{Slug, Viewer};
use crate::request;
use futures::prelude::*;
use indexmap::IndexMap;
use seed::fetch;

type RootDecoder = IndexMap<(), ()>;

pub fn delete<Ms: 'static>(
    viewer: Option<&Viewer>,
    slug: &Slug,
    f: fn(Result<(), Vec<String>>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new_api_request(&format!("articles/{}", slug.as_str()), viewer)
        .method(fetch::Method::Delete)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |_| ())
                .map_err(request::fail_reason_into_errors))
        })
}
