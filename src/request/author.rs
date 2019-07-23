use crate::entity::{Author, ErrorMessage, Username, Viewer};
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

#[allow(clippy::type_complexity)]
pub fn load<Ms: 'static>(
    viewer: Option<Viewer>,
    username: Username<'static>,
    f: fn(Result<Author, (Username<'static>, Vec<ErrorMessage>)>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new(&format!("profiles/{}", username.as_str()), viewer.as_ref()).fetch_json_data(
        move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |root_decoder| root_decoder.profile.into_author(viewer.map(Cow::Owned)))
                .map_err(request::fail_reason_into_errors)
                .map_err(move |errors| (username, errors)))
        },
    )
}
