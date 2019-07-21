use serde::Deserialize;
use crate::{form::settings as form, api, request};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    user: UserDto
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
// @TODO to dto folder?
struct UserDto {
    email: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
}

impl RootDto {
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

pub fn load<Ms: 'static>(
    credentials: Option<&api::Credentials>,
    f: fn(Result<form::Form, Vec<form::Problem>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        "user",
        credentials
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(RootDto::into_form)
                .map_err(request::fail_reason_into_problems)
            )
        })
}
