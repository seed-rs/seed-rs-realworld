use serde::{Serialize, Deserialize};
use crate::{session, article, request, dto};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    comment: dto::comment::CommentDto
}

// @TODO commentToSend and formsDTO solve somehow (move to dto folder?) + dto in request

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommentToSendDTOFields {
    body: String
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommentToSendDTO {
    comment: CommentToSendDTOFields
}

pub fn create_comment<Ms: 'static>(
    session: &session::Session,
    slug: &article::slug::Slug,
    text: String,
    f: fn(Result<article::comment::Comment<'static>, Vec<String>>) -> Ms
) -> impl Future<Item=Ms, Error=Ms>  {
    let session = session.clone();

    let dto = CommentToSendDTO {
        comment: CommentToSendDTOFields {
            body: text
        }
    };

    request::new_api_request(
        &format!("articles/{}/comments", slug.as_str()),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .method(fetch::Method::Post)
        .send_json(&dto)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map_err(request::fail_reason_into_errors)
                .and_then(move |root_dto| {
                    root_dto.comment.try_into_comment(session)
                        .map_err(|error| vec![error])
                })
            )
        })
}