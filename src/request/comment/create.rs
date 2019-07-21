use serde::Deserialize;
use crate::entity::{Credentials, article};
use crate::{request, coder::{decoder, encoder}};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    comment: decoder::Comment
}

pub fn create<Ms: 'static>(
    credentials: Option<Credentials>,
    slug: &article::slug::Slug,
    text: String,
    f: fn(Result<article::comment::Comment<'static>, Vec<String>>) -> Ms
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("articles/{}/comments", slug.as_str()),
        credentials.as_ref()
    )
        .method(fetch::Method::Post)
        .send_json(&encoder::Comment::new(text))
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map_err(request::fail_reason_into_errors)
                .and_then(move |root_decoder| {
                    root_decoder.comment.try_into_comment(credentials)
                        .map_err(|error| vec![error])
                })
            )
        })
}