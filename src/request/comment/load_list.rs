use serde::Deserialize;
use crate::entity::{Credentials, article};
use crate::{request, coder::decoder, logger};
use futures::prelude::*;
use seed::fetch;
use std::collections::VecDeque;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    comments: VecDeque<decoder::Comment>,
}

impl RootDecoder {
    fn into_comments<'a>(self, credentials: Option<Credentials>) -> VecDeque<article::comment::Comment<'a>> {
        self.comments.into_iter().filter_map(|comment_decoder| {
            match comment_decoder.try_into_comment(credentials.clone()) {
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
    credentials: Option<Credentials>,
    slug: &article::slug::Slug,
    f: fn(Result<VecDeque<article::comment::Comment<'static>>, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("articles/{}/comments", slug.as_str()),
        credentials.as_ref()
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |root_decoder| root_decoder.into_comments(credentials))
                .map_err(request::fail_reason_into_errors)
            )
        })
}