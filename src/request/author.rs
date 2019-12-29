use crate::{
    coder::decoder,
    entity::{Author, ErrorMessage, Username, Viewer},
    request,
};
use seed::fetch::ResponseDataResult;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    profile: decoder::Author,
}

#[allow(clippy::type_complexity)]
pub async fn load<Ms: 'static>(
    viewer: Option<Viewer>,
    username: Username<'static>,
    f: fn(Result<Author, (Username<'static>, Vec<ErrorMessage>)>) -> Ms,
) -> Result<Ms, Ms> {
    request::new(&format!("profiles/{}", username.as_str()), viewer.as_ref())
        .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |root_decoder| root_decoder.profile.into_author(viewer.map(Cow::Owned)))
                .map_err(request::fail_reason_into_errors)
                .map_err(move |errors| (username, errors)))
        })
        .await
}
