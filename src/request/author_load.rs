use serde::Deserialize;
use crate::{username, session, author, request, dto};
use futures::prelude::*;
use seed::fetch;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RootDto {
    profile: dto::author::AuthorDTO
}

pub fn load_author<Ms: 'static>(
    session: session::Session,
    username: username::Username<'static>,
    f: fn(Result<author::Author<'static>, (username::Username<'static>, Vec<String>)>) -> Ms,
) -> impl Future<Item=Ms, Error=Ms>  {
    let username = username.clone();
    let session = session.clone();

    request::new_api_request(
        &format!("profiles/{}", username.as_str()),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<RootDto>| {
            f(data_result
                .map(move |root_dto| root_dto.profile.into_author(session))
                .map_err(request::fail_reason_into_errors)
                .map_err(move |errors| (username, errors))
            )
        })
}