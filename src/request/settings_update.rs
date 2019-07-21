use serde::Deserialize;
use crate::{viewer, form::settings as form, session, request, dto};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    user: dto::viewer::Viewer
}

pub fn update_settings<Ms: 'static>(
    session: &session::Session,
    valid_form: &form::ValidForm,
    f: fn(Result<viewer::Viewer, Vec<form::Problem>>) -> Ms
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        "user",
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .method(fetch::Method::Put)
        .send_json(&valid_form.dto())
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(|root_dto| root_dto.user.into_viewer())
                .map_err(fail_reason_to_problems)
            )
        })
}

fn fail_reason_to_problems(fail_reason: fetch::FailReason<RootDto>) -> Vec<form::Problem> {
    string_errors_to_problems(request::fail_reason_into_errors(fail_reason))
}

fn string_errors_to_problems(errors: Vec<String>) -> Vec<form::Problem> {
    errors.into_iter().map(form::Problem::new_server_error).collect()
}