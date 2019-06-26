use serde::Deserialize;
use crate::{viewer, avatar, username, api, login_form};
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
    id: i32,
    email: String,
    created_at: String,
    updated_at: String,
    username: String,
    bio: Option<String>,
    image: Option<String>,
    token: String,
}

impl ServerData {
    fn into_viewer(self) -> viewer::Viewer {
        viewer::Viewer {
            avatar: avatar::Avatar::new(self.user.image),
            credentials: api::Credentials {
                username: username::Username::new(self.user.username),
                auth_token: self.user.token
            }
        }
    }
}

pub fn login<Ms: 'static>(valid_form: &login_form::ValidForm, f: fn(Result<viewer::Viewer, Vec<login_form::Problem>>) -> Ms) -> impl Future<Item=Ms, Error=Ms>  {
    fetch::Request::new("https://conduit.productionready.io/api/users/login".into())
        .method(fetch::Method::Post)
        .timeout(5000)
        .send_json(valid_form)
        .fetch_string(move |fetch_object| {
            f(process_fetch_object(fetch_object))
        })
}

fn process_fetch_object(fetch_object: fetch::FetchObject<String>) -> Result<viewer::Viewer, Vec<login_form::Problem>> {
    match fetch_object.result {
        Err(request_error) => {
            Err(vec![login_form::Problem::ServerError("Request error".into())])
        },
        Ok(response) => {
            if response.status.is_ok() {
                    let viewer =
                        response
                            .data
                            .and_then(|string| {
                                serde_json::from_str::<ServerData>(string.as_str())
                                    .map_err(|error| {
                                        fetch::DataError::SerdeError(Rc::new(error))
                                    })
                            })
                            .map(|server_data| {
                                server_data.into_viewer()
                            });

                    match viewer {
                        Ok(viewer) => {
                            Ok(viewer)
                        },
                        Err(data_error) => {
                            Err(vec![login_form::Problem::ServerError("Data error".into())])
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
                                login_form::Problem::ServerError(message)
                            }).collect();
                        Err(problems)
                    },
                    Err(data_error) => {
                        Err(vec![login_form::Problem::ServerError("Data error".into())])
                    }
                }
            }
        }
    }
}
