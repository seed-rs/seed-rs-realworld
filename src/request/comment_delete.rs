use crate::{session, article, comment_id, request};
use futures::prelude::*;
use seed::fetch;
use indexmap::IndexMap;

type ServerData = IndexMap<(), ()>;

pub fn delete_comment<Ms: 'static>(
    session: &session::Session,
    slug: &article::slug::Slug,
    comment_id: comment_id::CommentId,
    f: fn(Result<comment_id::CommentId, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let mut request = fetch::Request::new(
        format!(
            "https://conduit.productionready.io/api/articles/{}/comments/{}",
            slug.as_str(),
            comment_id.as_str()
        )
    )
        .method(fetch::Method::Delete)
        .timeout(5000);

    if let Some(viewer) = session.viewer() {
        let auth_token = viewer.credentials.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }

    request.fetch_json_data(move |data_result: fetch::ResponseDataResult<ServerData>| {
        f(data_result
            .map(move |_| comment_id)
            .map_err(request::fail_reason_into_errors)
        )
    })
}