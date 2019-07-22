use crate::entity::{Author, Username, Viewer};
use crate::{coder::decoder, request};
use futures::prelude::*;
use seed::fetch;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    profile: decoder::Author,
}

pub fn unfollow<Ms: 'static>(
    viewer: Option<Viewer>,
    username: &Username,
    f: fn(Result<Author, Vec<String>>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new_api_request(
        &format!("profiles/{}/follow", username.as_str()),
        viewer.as_ref(),
    )
    .method(fetch::Method::Delete)
    .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
        f(data_result
            .map(move |root_decoder| root_decoder.profile.into_author(viewer.map(Cow::Owned)))
            .map_err(request::fail_reason_into_errors))
    })
}
