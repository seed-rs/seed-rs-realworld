use crate::{
    coder::decoder,
    entity::{Author, ErrorMessage, Username, Viewer},
    request,
};
use seed::fetch::{Method, ResponseDataResult};
use serde::Deserialize;
use std::{borrow::Cow, future::Future};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    profile: decoder::Author,
}

pub fn unfollow<Ms: 'static>(
    viewer: Option<Viewer>,
    username: &Username<'_>,
    f: fn(Result<Author, Vec<ErrorMessage>>) -> Ms,
) -> impl Future<Output = Result<Ms, Ms>> {
    request::new(
        &format!("profiles/{}/follow", username.as_str()),
        viewer.as_ref(),
    )
    .method(Method::Delete)
    .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
        f(data_result
            .map(move |root_decoder| root_decoder.profile.into_author(viewer.map(Cow::Owned)))
            .map_err(request::fail_reason_into_errors))
    })
}
