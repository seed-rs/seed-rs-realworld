use serde::Deserialize;
use crate::entity::{username, Viewer, Author};
use crate::{request, coder::decoder};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    profile: decoder::Author
}

pub fn follow<Ms: 'static>(
    viewer: Option<Viewer>,
    username: &username::Username,
    f: fn(Result<Author, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        &format!("profiles/{}/follow", username.as_str()),
        viewer.as_ref()
    )
        .method(fetch::Method::Post)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(move |root_decoder| root_decoder.profile.into_author(viewer))
                .map_err(request::fail_reason_into_errors)
            )
        })
}