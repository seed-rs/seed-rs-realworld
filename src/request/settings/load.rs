use crate::entity::{
    form::settings::{Form, Problem},
    Viewer,
};
use crate::{coder::decoder, request};
use futures::prelude::*;
use seed::fetch;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    user: decoder::Settings,
}

pub fn load<Ms: 'static>(
    viewer: Option<&Viewer>,
    f: fn(Result<Form, Vec<Problem>>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new_api_request("user", viewer).fetch_json_data(
        move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(|root_decoder| root_decoder.user.into_form())
                .map_err(request::fail_reason_into_problems))
        },
    )
}
