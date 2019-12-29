use crate::{
    entity::{
        article::tag::{IntoTags, Tag},
        ErrorMessage,
    },
    request,
};
use seed::fetch::ResponseDataResult;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    tags: Vec<String>,
}

pub async fn load_list<Ms: 'static>(
    f: fn(Result<Vec<Tag>, Vec<ErrorMessage>>) -> Ms,
) -> Result<Ms, Ms> {
    request::new("tags", None)
        .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
            f(data_result
                .map(|root_decoder| root_decoder.tags.into_tags())
                .map_err(request::fail_reason_into_errors))
        })
        .await
}
