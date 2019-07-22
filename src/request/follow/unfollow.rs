use serde::Deserialize;
use crate::entity::{Username, Author, Credentials};
use crate::{request, coder::decoder};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    profile: decoder::Author
}

pub fn unfollow<Ms: 'static>(
    credentials: Option<Credentials>,
    username: &Username,
    f: fn(Result<Author<'static>, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("profiles/{}/follow", username.as_str()),
        credentials.as_ref()
    )
        .method(fetch::Method::Delete)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |root_decoder| root_decoder.profile.into_author(credentials))
                .map_err(request::fail_reason_into_errors)
            )
        })
}