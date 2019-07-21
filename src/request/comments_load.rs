use serde::Deserialize;
use crate::{session, article, logger, request, dto};
use futures::prelude::*;
use seed::fetch;
use std::collections::VecDeque;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    comments: VecDeque<dto::comment::CommentDto>,
}

impl RootDto {
    fn into_comments<'a>(self, session: session::Session) -> VecDeque<article::comment::Comment<'a>> {
        self.comments.into_iter().filter_map(|comment_dto| {
            // @TODO without clone / more effective?
            match comment_dto.try_into_comment(session.clone()) {
                Ok(comment) => Some(comment),
                Err(error) => {
                    logger::error(error);
                    None
                }
            }
        }).collect()
    }
}

pub fn load_comments<Ms: 'static>(
    session: session::Session,
    slug: &article::slug::Slug,
    f: fn(Result<VecDeque<article::comment::Comment<'static>>, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let session = session.clone();

    request::new_api_request(
        &format!("articles/{}/comments", slug.as_str()),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(move |root_dto| root_dto.into_comments(session))
                .map_err(request::fail_reason_into_errors)
            )
        })
}