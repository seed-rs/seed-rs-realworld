use serde::Deserialize;
use crate::{viewer, avatar, api, form::login as form, request};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    user: ServerDataFields
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerDataFields {
    username: String,
    image: Option<String>,
    token: String,
}

impl ServerData {
    fn into_viewer(self) -> viewer::Viewer {
        viewer::Viewer {
            avatar: avatar::Avatar::new(self.user.image),
            credentials: api::Credentials {
                username: self.user.username.into(),
                auth_token: self.user.token
            }
        }
    }
}

pub fn login<Ms: 'static>(
    valid_form: &form::ValidForm,
    f: fn(Result<viewer::Viewer, Vec<form::Problem>>) -> Ms
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request("users/login", None)
        .method(fetch::Method::Post)
        .send_json(&valid_form.dto())
        .fetch_json_data(move |data_result| {
            f(data_result
                .map(ServerData::into_viewer)
                .map_err(fail_reason_to_problems)
            )
        })
}

fn fail_reason_to_problems(fail_reason: fetch::FailReason<ServerData>) -> Vec<form::Problem> {
    string_errors_to_problems(request::fail_reason_into_errors(fail_reason))
}

fn string_errors_to_problems(errors: Vec<String>) -> Vec<form::Problem> {
    errors.into_iter().map(form::Problem::new_server_error).collect()
}
