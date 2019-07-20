use serde::Deserialize;
use crate::{avatar, form::article_editor as form, session, article, author, profile, request};
use futures::prelude::*;
use seed::fetch;
use std::convert::TryInto;
use article::tag::IntoTags;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerData {
    article: ServerDataFields
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerDataFields {
    title: String,
    slug: String,
    body: String,
    created_at: String,
    updated_at: String,
    tag_list: Vec<String>,
    description: String,
    author: ServerDataFieldAuthor,
    favorited: bool,
    favorites_count: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerDataFieldAuthor {
    username: String,
    bio: Option<String>,
    image: String,
    following: bool,
}

impl ServerDataFieldAuthor {
    fn into_author(self, session: session::Session) -> author::Author<'static> {
        let username = self.username.into();
        let profile = profile::Profile {
            bio: self.bio,
            avatar: avatar::Avatar::new(Some(self.image)),
        };

        if let Some(viewer) = session.viewer() {
            if &username == viewer.username() {
                return author::Author::IsViewer(viewer.credentials.clone(), profile)
            }
        }

        if self.following {
            author::Author::Following(
                author::FollowedAuthor(username, profile)
            )
        } else {
            author::Author::NotFollowing(
                author::UnfollowedAuthor(username, profile)
            )
        }
    }
}

impl ServerData {
    fn try_into_article(self, session: session::Session) -> Result<article::Article, String> {
        let created_at = self.article.created_at.try_into()?;
        let updated_at = self.article.updated_at.try_into()?;

        Ok(article::Article {
            title: self.article.title,
            slug: self.article.slug.into(),
            body: self.article.body.into(),
            created_at,
            updated_at,
            tag_list: self.article.tag_list.into_tags(),
            description: self.article.description,
            author: self.article.author.into_author(session),
            favorited: self.article.favorited,
            favorites_count: self.article.favorites_count,
        })
    }
}

pub fn update_article<Ms: 'static>(
    session: &session::Session,
    valid_form: &form::ValidForm,
    slug: &article::slug::Slug,
    f: fn(Result<article::Article, Vec<form::Problem>>) -> Ms
) -> impl Future<Item=Ms, Error=Ms>  {
    let session = session.clone();

    request::new_api_request(
        &format!("articles/{}", slug.as_str()),
        session.viewer().map(|viewer| &viewer.credentials)
    )
        .method(fetch::Method::Put)
        .send_json(&valid_form.dto())
        .fetch_json_data(move |data_result: fetch::ResponseDataResult<ServerData>| {
            f(data_result
                .map_err(fail_reason_to_problems)
                .and_then(move |server_data| {
                    server_data.try_into_article(session)
                        .map_err(|error| vec![form::Problem::new_server_error(error)])
                })
            )
        })
}

pub fn fail_reason_to_problems(fail_reason: fetch::FailReason<ServerData>) -> Vec<form::Problem> {
    string_errors_to_problems(request::fail_reason_into_errors(fail_reason))
}

pub fn string_errors_to_problems(errors: Vec<String>) -> Vec<form::Problem> {
    errors.into_iter().map(form::Problem::new_server_error).collect()
}