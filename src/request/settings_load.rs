use serde::Deserialize;
use crate::{form::settings as form, session};
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;
use std::rc::Rc;

#[derive(Deserialize)]
struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    user: ServerDataFields
}

#[derive(Deserialize)]
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
        "https://conduit.productionready.io/api/user".into()
    ).timeout(5000);

    if let Some(viewer) = session.viewer() {
        let auth_token = viewer.credentials.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }

    request.fetch_string(move |fetch_object| {
        f(process_fetch_object(fetch_object))
    })
}

fn process_fetch_object(fetch_object: fetch::FetchObject<String>) -> Result<form::Form, Vec<form::Problem>> {
    match fetch_object.result {
        Err(_) => {
            Err(vec![form::Problem::new_server_error("Request error")])
        },
        Ok(response) => {
            if response.status.is_ok() {
                    let form =
                        response
                            .data
                            .and_then(|string| {
                                serde_json::from_str::<ServerData>(string.as_str())
                                    .map_err(|error| {
                                        fetch::DataError::SerdeError(Rc::new(error))
                                    })
                            })
                            .map(|server_data| {
                                server_data.into_form()
                            });

                    match form {
                        Ok(form) => {
                            Ok(form)
                        },
                        Err(_) => {
                            Err(vec![form::Problem::new_server_error("Data error")])
                        }
                    }
            } else {
                let error_messages: Result<Vec<String>, fetch::DataError> =
                    response
                        .data
                        .and_then(|string| {
                            serde_json::from_str::<ServerErrorData>(string.as_str())
                                .map_err(|error| {
                                    fetch::DataError::SerdeError(Rc::new(error))
                                })
                        }).and_then(|server_error_data| {
                        Ok(server_error_data.errors.into_iter().map(|(field, errors)| {
                            format!("{} {}", field, errors.join(", "))
                        }).collect())
                    });
                match error_messages {
                    Ok(error_messages) => {
                        let problems = error_messages
                            .into_iter()
                            .map(|message| {
                                form::Problem::new_server_error(message)
                            }).collect();
                        Err(problems)
                    },
                    Err(_) => {
                        Err(vec![form::Problem::new_server_error("Data error")])
                    }
                }
            }
        }
    }
}
