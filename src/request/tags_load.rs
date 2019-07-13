use serde::Deserialize;
use crate::{viewer, avatar, username, api, session, article, page, paginated_list, author, profile, timestamp};
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;
use std::rc::Rc;
use std::convert::TryFrom;
use std::convert::TryInto;

const ARTICLES_PER_PAGE: usize = 5;

#[derive(Deserialize)]
struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServerData {
    tags: Vec<String>,
}

impl ServerData {
    fn into_tags(self) -> Vec<article::tag::Tag> {
        self.tags.into_iter().map(article::tag::Tag::new).collect()
    }
}

pub fn load_tags<Ms: 'static>(
    f: fn(Result<Vec<article::tag::Tag>, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    fetch::Request::new(
        "https://conduit.productionready.io/api/tags".into()
    )
        .timeout(5000)
        .fetch_string(move |fetch_object| {
            f(process_fetch_object(fetch_object))
        })
}

fn process_fetch_object(
    fetch_object: fetch::FetchObject<String>
) -> Result<Vec<article::tag::Tag>, Vec<String>> {
    match fetch_object.result {
        Err(request_error) => {
            Err(vec!["Request error".into()])
        },
        Ok(response) => {
            if response.status.is_ok() {
                    let paginated_list =
                        response
                            .data
                            .and_then(|string| {
                                serde_json::from_str::<ServerData>(string.as_str())
                                    .map_err(|error| {
                                        fetch::DataError::SerdeError(Rc::new(error))
                                    })
                            })
                            .map(|server_data| {
                                server_data.into_tags()
                            });

                    match paginated_list {
                        Ok(paginated_list) => {
                            Ok(paginated_list)
                        },
                        Err(data_error) => {
                            Err(vec!["Data error".into()])
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
                        Err(error_messages)
                    },
                    Err(data_error) => {
                        Err(vec!["Data error".into()])
                    }
                }
            }
        }
    }
}
