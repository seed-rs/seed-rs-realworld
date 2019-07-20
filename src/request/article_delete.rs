use crate::{session, article, request};
use indexmap::IndexMap;
use futures::prelude::*;
use seed::fetch;

type ServerData = IndexMap<(), ()>;

pub fn delete_article<Ms: 'static>(
    session: &session::Session,
    slug: &article::slug::Slug,
    f: fn(Result<(), Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let mut request = fetch::Request::new(
        format!("https://conduit.productionready.io/api/articles/{}", slug.as_str())
    )
        .method(fetch::Method::Delete)
        .timeout(5000);

    if let Some(viewer) = session.viewer() {
        let auth_token = viewer.credentials.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }

    request.fetch_json_data(move |data_result: fetch::ResponseDataResult<ServerData>| {
        f(data_result
            .map(move |_| ())
            .map_err(request::fail_reason_into_errors)
        )
    })
}