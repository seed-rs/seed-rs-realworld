use crate::entity::{Comment, ErrorMessage, Slug, Viewer};
use crate::{
    coder::{decoder, encoder},
    request,
};
use futures::prelude::*;
use seed::fetch;
use serde::Deserialize;
use std::borrow::Cow;

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
) -> impl Future<Item = Ms, Error = Ms> {
    request::new(
        &format!("articles/{}/comments", slug.as_str()),
        viewer.as_ref(),
    )
    .method(fetch::Method::Post)
    .send_json(&encoder::Comment::new(text))
    .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
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
