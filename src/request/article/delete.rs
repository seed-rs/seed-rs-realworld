use std::future::Future;

use indexmap::IndexMap;
use seed::fetch::{Method, ResponseDataResult};

use crate::{
    entity::{ErrorMessage, Slug, Viewer},
    request,
};

type RootDecoder = IndexMap<(), ()>;

pub fn delete<Ms: 'static>(
    viewer: Option<&Viewer>,
    slug: &Slug,
    f: fn(Result<(), Vec<ErrorMessage>>) -> Ms,
) -> impl Future<Output = Result<Ms, Ms>> {
    request::new(&format!("articles/{}", slug.as_str()), viewer)
        .method(Method::Delete)
        .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |_| ())
                .map_err(request::fail_reason_into_errors))
        })
}
