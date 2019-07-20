use serde::Deserialize;
use crate::{form::settings as form, session, request};
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
    email: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

impl ServerData {
    fn into_form(self) -> form::Form {
        let fields: Vec<form::Field> = vec![
            form::Field::Avatar(self.user.image.unwrap_or_default()),
            form::Field::Username(self.user.username),
            form::Field::Bio(self.user.bio.unwrap_or_default()),
            form::Field::Email(self.user.email),
            form::Field::Password(String::default()),
        ];
        form::Form::new(fields)
    }
}

pub fn load_settings<Ms: 'static>(
    session: &session::Session,
    f: fn(Result<form::Form, Vec<form::Problem>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {

    let mut request = fetch::Request::new(
        "https://conduit.productionready.io/api/user"
    ).timeout(5000);

    if let Some(viewer) = session.viewer() {
        let auth_token = viewer.credentials.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }

    request.fetch_json_data(move |data_result| {
        f(data_result
            .map(ServerData::into_form)
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