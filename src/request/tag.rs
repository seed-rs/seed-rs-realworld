use crate::entity::article::tag::{IntoTags, Tag};
use crate::request;
use futures::prelude::*;
use seed::fetch;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    tags: Vec<String>,
}

pub fn load_list<Ms: 'static>(
    f: fn(Result<Vec<Tag>, Vec<String>>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new("tags", None).fetch_json_data(
        move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(|root_decoder| root_decoder.tags.into_tags())
                .map_err(request::fail_reason_into_errors))
        },
    )
}
