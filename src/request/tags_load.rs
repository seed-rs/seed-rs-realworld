use serde::Deserialize;
use crate::{article, request};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
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
    request::new_api_request("tags",None)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<ServerData>| {
            f(data_result
                .map(ServerData::into_tags)
                .map_err(request::fail_reason_into_errors)
            )
        })
}
