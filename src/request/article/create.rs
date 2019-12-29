use std::{borrow::Cow, future::Future};

use seed::fetch::{Method, ResponseDataResult};
use serde::Deserialize;

use crate::{
    coder::decoder,
    entity::{
        form::article_editor::{Problem, ValidForm},
        Article, Viewer,
    },
    request,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    article: decoder::Article,
}

pub fn create<Ms: 'static>(
    viewer: Option<Viewer>,
    valid_form: &ValidForm,
    f: fn(Result<Article, Vec<Problem>>) -> Ms,
) -> impl Future<Output = Result<Ms, Ms>> {
    request::new("articles", viewer.as_ref())
        .method(Method::Post)
        .send_json(&valid_form.to_encoder())
        .fetch_json_data(move |data_result: ResponseDataResult<RootDecoder>| {
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
