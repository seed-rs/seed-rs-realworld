use crate::{
    coder::decoder,
    entity::{
        form::register::{Problem, ValidForm},
        Viewer,
    },
    request,
};
use seed::fetch::{Method, ResponseDataResult};
use serde::Deserialize;
use std::future::Future;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    user: decoder::Viewer,
}

pub fn register<Ms: 'static>(
    valid_form: &ValidForm,
    f: fn(Result<Viewer, Vec<Problem>>) -> Ms,
) -> impl Future<Output = Result<Ms, Ms>> {
    request::new("users", None)
        .method(Method::Post)
        .send_json(&valid_form.to_encoder())
        .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(|root_decoder| root_decoder.user.into_viewer())
                .map_err(request::fail_reason_into_problems))
        })
}
