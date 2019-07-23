use crate::entity::{
    form::article_editor::{Problem, ValidForm},
    Article, Viewer,
};
use crate::{coder::decoder, request};
use futures::prelude::*;
use seed::fetch;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    article: decoder::Article,
}

pub fn create<Ms: 'static>(
    viewer: Option<Viewer>,
    valid_form: &ValidForm,
    f: fn(Result<Article, Vec<Problem>>) -> Ms,
) -> impl Future<Item = Ms, Error = Ms> {
    request::new("articles", viewer.as_ref())
        .method(fetch::Method::Post)
        .send_json(&valid_form.to_encoder())
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map_err(request::fail_reason_into_problems)
                .and_then(move |root_decoder| {
                    root_decoder
                        .article
                        .try_into_article(viewer.map(Cow::Owned))
                        .map_err(|error| vec![Problem::new_server_error(error.into_inner())])
                }))
        })
}
