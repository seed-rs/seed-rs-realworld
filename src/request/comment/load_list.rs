use serde::Deserialize;
use crate::{api, article, logger, request, dto};
use futures::prelude::*;
use seed::fetch;
use std::collections::VecDeque;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    comments: VecDeque<dto::comment::CommentDto>,
}

impl RootDto {
    fn into_comments<'a>(self, credentials: Option<api::Credentials>) -> VecDeque<article::comment::Comment<'a>> {
        self.comments.into_iter().filter_map(|comment_dto| {
            // @TODO without clone / more effective?
            match comment_dto.try_into_comment(credentials.clone()) {
                Ok(comment) => Some(comment),
                Err(error) => {
                    logger::error(error);
                    None
                }
            }
        }).collect()
    }
}

pub fn load_list<Ms: 'static>(
    credentials: Option<api::Credentials>,
    slug: &article::slug::Slug,
    f: fn(Result<VecDeque<article::comment::Comment<'static>>, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("articles/{}/comments", slug.as_str()),
        credentials.as_ref()
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(move |root_dto| root_dto.into_comments(credentials))
                .map_err(request::fail_reason_into_errors)
            )
        })
}