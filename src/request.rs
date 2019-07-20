use seed::fetch;
use serde_json;
use indexmap::IndexMap;
use serde::Deserialize;
use std::fmt::Debug;
use crate::logger;

pub mod article_load;
pub mod article_article_load;
pub mod article_update;
pub mod article_create;
pub mod article_delete;
pub mod login;
pub mod register;
pub mod settings_load;
pub mod settings_update;
pub mod author_load;
pub mod feed_load;
pub mod home_feed_load;
pub mod follow;
pub mod unfollow;
pub mod favorite;
pub mod unfavorite;
pub mod tags_load;
pub mod comment_create;
pub mod comment_delete;
pub mod comments_load;

#[derive(Deserialize)]
pub struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

pub fn decode_server_errors(json: String) -> Result<Vec<String>, serde_json::Error> {
    let server_error_data = serde_json::from_str::<ServerErrorData>(json.as_str())?;
    Ok(server_error_data
        .errors
        .into_iter()
        .map(|(field, errors)| {
            format!("{} {}", field, errors.join(", "))
        }).collect())
}

pub fn fail_reason_into_errors<T: Debug>(fail_reason: fetch::FailReason<T>) -> Vec<String> {
    match fail_reason {
        fetch::FailReason::RequestError(request_error, _) => {
            logger::error(request_error);
            vec!["Request error".into()]
        }
        fetch::FailReason::DataError(data_error, _) => {
            logger::error(data_error);
            vec!["Data error".into()]
        }
        fetch::FailReason::Status(_, fetch_object) => {
            let response = fetch_object.result.unwrap();
            match response.data {
                Err(fetch::DataError::SerdeError(_, json)) => {
                    decode_server_errors(json)
                        .unwrap_or_else(|serde_error|{
                            logger::error(serde_error);
                            vec!["Data error".into()]
                        })
                }
                data => {
                    logger::error(data);
                    vec!["Data error".into()]
                }
            }
        }
    }
}