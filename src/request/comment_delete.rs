use serde::Deserialize;
use crate::{session, article, comment_id};
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;
use std::rc::Rc;

#[derive(Deserialize)]
struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

pub fn delete_comment<Ms: 'static>(
    session: &session::Session,
    slug: &article::slug::Slug,
    comment_id: comment_id::CommentId,
    f: fn(Result<comment_id::CommentId, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let mut request = fetch::Request::new(
        format!(
            "https://conduit.productionready.io/api/articles/{}/comments/{}",
            slug.as_str(),
            comment_id.as_str()
        )
    )
        .method(fetch::Method::Delete)
        .timeout(5000);

    if let Some(viewer) = session.viewer() {
        let auth_token = viewer.credentials.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }

    request.fetch_string(move |fetch_object| {
        f(process_fetch_object(comment_id, fetch_object))
    })
}

fn process_fetch_object(
    comment_id: comment_id::CommentId,
    fetch_object: fetch::FetchObject<String>
) -> Result<comment_id::CommentId, Vec<String>> {
    match fetch_object.result {
        Err(_) => {
            Err(vec!["Request error".into()])
        },
        Ok(response) => {
            if response.status.is_ok() {
                Ok(comment_id)
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
                    Err(_) => {
                        Err(vec!["Data error".into()])
                    }
                }
            }
        }
    }
}
