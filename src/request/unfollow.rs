use serde::Deserialize;
use crate::{username, session, author, request, dto};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    profile: dto::author::AuthorDTO
}

pub fn unfollow<Ms: 'static>(
    session: session::Session,
    username: username::Username<'static>,
    f: fn(Result<author::Author<'static>, Vec<String>>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let username = username.clone();
    let session = session.clone();

    request::new_api_request(
        &format!("profiles/{}/follow", username.as_str()),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .method(fetch::Method::Delete)
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(move |root_dto| root_dto.profile.into_author(session))
                .map_err(request::fail_reason_into_errors)
            )
        })
}