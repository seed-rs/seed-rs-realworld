use crate::entity::{username, Author, ErrorMessage, Viewer};
use crate::{coder::decoder, request};
use futures::prelude::*;
use seed::fetch::{Method, ResponseDataResult};
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    profile: decoder::Author,
}

pub fn follow<Ms: 'static>(
    viewer: Option<Viewer>,
    username: &username::Username,
    f: fn(Result<Author, Vec<ErrorMessage>>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new(
        &format!("profiles/{}/follow", username.as_str()),
        viewer.as_ref(),
    )
    .method(Method::Post)
    .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
        f(data_result
            .map(move |root_decoder| root_decoder.profile.into_author(viewer.map(Cow::Owned)))
            .map_err(request::fail_reason_into_errors))
    })
}
