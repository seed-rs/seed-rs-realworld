use serde::Deserialize;
use crate::entity::{form::article_editor::{ValidForm, Problem}, Credentials, Article};
use crate::{request, coder::decoder};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDecoder {
    article: decoder::Article
}

pub fn create<Ms: 'static>(
    credentials: Option<Credentials>,
    valid_form: &ValidForm,
    f: fn(Result<Article, Vec<Problem>>) -> Ms
) -> impl Future<Item=Ms, Error=Ms>  {
    request::new_api_request(
        "articles",
        credentials.as_ref()
    )
        .method(fetch::Method::Post)
        .send_json(&valid_form.to_encoder())
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDecoder>| {
            f(data_result
                .map_err(request::fail_reason_into_problems)
                .and_then(move |root_decoder| {
                    root_decoder.article.try_into_article(credentials)
                        .map_err(|error| vec![Problem::new_server_error(error)])
                })
            )
        })
}



