use serde::Deserialize;
use crate::{api, article, dto};
use std::convert::TryInto;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommentDto {
    id: usize,
    created_at: String,
    updated_at: String,
    body: String,
    author: dto::author::AuthorDTO,
}

impl CommentDto {
    pub fn try_into_comment<'a>(self, credentials: Option<api::Credentials>,) -> Result<article::comment::Comment<'a>, String> {
        let created_at = self.created_at.try_into()?;
        let updated_at = self.updated_at.try_into()?;

        Ok(article::comment::Comment {
            id: self.id.to_string().into(),
            body: self.body,
            created_at,
            updated_at,
            author: self.author.into_author(credentials),
        })
    }
}