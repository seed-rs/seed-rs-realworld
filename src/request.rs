use crate::{
    coder::decoder,
    entity::{form::Problem, ErrorMessage, Viewer},
    logger,
};
use seed::fetch;
use serde_json;
use std::fmt::Debug;

pub mod article;
pub mod author;
pub mod comment;
pub mod favorite;
pub mod feed;
pub mod follow;
pub mod login;
pub mod register;
pub mod settings;
pub mod tag;

static BASE_API_URL: &str = "https://conduit.productionready.io/api";
const TIMEOUT: u32 = 5000;

pub fn new(path: &str, viewer: Option<&Viewer>) -> fetch::Request {
    let mut request = fetch::Request::new(format!("{}/{}", BASE_API_URL, path)).timeout(TIMEOUT);

    if let Some(viewer) = viewer {
        let auth_token = viewer.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }
    request
}

pub fn fail_reason_into_problems<T: Debug>(fail_reason: fetch::FailReason<T>) -> Vec<Problem> {
    fail_reason_into_errors(fail_reason)
        .into_iter()
        .map(|error| Problem::new_server_error(error.into_inner()))
        .collect()
}

pub fn fail_reason_into_errors<T: Debug>(fail_reason: fetch::FailReason<T>) -> Vec<ErrorMessage> {
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
            // response isn't ok, but maybe contains error messages - try to decode them:
            match fetch_object.result.unwrap().data {
                Err(fetch::DataError::SerdeError(_, json)) => decode_server_errors(&json)
                    .unwrap_or_else(|serde_error| {
                        logger::error(serde_error);
                        vec!["Data error".into()]
                    }),
                data => {
                    logger::error(data);
                    vec!["Data error".into()]
                }
            }
        }
    }
}

// ====== PRIVATE ======

fn decode_server_errors(json: &str) -> Result<Vec<ErrorMessage>, serde_json::Error> {
    serde_json::from_str::<decoder::ErrorMessages>(json)
        .map(decoder::ErrorMessages::into_error_messages)
}
