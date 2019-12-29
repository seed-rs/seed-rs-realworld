use std::{borrow::Cow, future::Future};

use seed::fetch::{Method, ResponseDataResult};
use serde::Deserialize;

use crate::{
    coder::{decoder, encoder},
    entity::{Comment, ErrorMessage, Slug, Viewer},
    request,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    comment: decoder::Comment,
}

pub fn create<Ms: 'static>(
    viewer: Option<Viewer>,
    slug: &Slug,
    text: String,
    f: fn(Result<Comment, Vec<ErrorMessage>>) -> Ms,
) -> impl Future<Output = Result<Ms, Ms>> {
    request::new(
        &format!("articles/{}/comments", slug.as_str()),
        viewer.as_ref(),
    )
    .method(Method::Post)
    .send_json(&encoder::Comment::new(text))
    .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
        f(data_result
            .map_err(request::fail_reason_into_errors)
            .and_then(move |root_decoder| {
                root_decoder
                    .comment
                    .try_into_comment(viewer.map(Cow::Owned))
                    .map_err(|error| vec![error])
            }))
    })
}
