use serde::Deserialize;
use crate::{viewer, form::settings as form, api, request, dto};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    user: dto::viewer::Viewer
}

pub fn update<Ms: 'static>(
    credentials: Option<&api::Credentials>,
    valid_form: &form::ValidForm,
    f: fn(Result<viewer::Viewer, Vec<form::Problem>>) -> Ms
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        "user",
        credentials
    )
        .method(fetch::Method::Put)
        .send_json(&valid_form.dto())
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(|root_dto| root_dto.user.into_viewer())
                .map_err(request::fail_reason_into_problems)
            )
        })
}
