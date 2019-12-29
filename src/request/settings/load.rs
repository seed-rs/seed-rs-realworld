use crate::{
    coder::decoder,
    entity::{
        form::settings::{Form, Problem},
        Viewer,
    },
    request,
};
use seed::fetch::ResponseDataResult;
use serde::Deserialize;
use std::future::Future;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    user: decoder::Settings,
}

pub fn load<Ms: 'static>(
    viewer: Option<&Viewer>,
    f: fn(Result<Form, Vec<Problem>>) -> Ms,
) -> impl Future<Output = Result<Ms, Ms>> {
    request::new("user", viewer).fetch_json_data(
        move |data_result: ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(|root_decoder| root_decoder.user.into_form())
                .map_err(request::fail_reason_into_problems))
        },
    )
}
