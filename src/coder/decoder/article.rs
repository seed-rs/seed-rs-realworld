use serde::Deserialize;
use crate::entity::{self, Viewer};
use crate::coder::decoder;
use crate::entity::article::tag::IntoTags;
use std::convert::TryInto;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    title: String,
    slug: String,
    body: String,
    created_at: String,
    updated_at: String,
    tag_list: Vec<String>,
    description: String,
    author: decoder::Author,
    favorited: bool,
    favorites_count: usize,
}

impl Article {
    pub fn try_into_article(self, viewer: Option<Viewer>,) -> Result<entity::Article, String> {
        let created_at = self.created_at.try_into()?;
        let updated_at = self.updated_at.try_into()?;

        Ok(entity::Article {
            title: self.title,
            slug: self.slug.into(),
            body: self.body.into(),
            created_at,
            updated_at,
            tag_list: self.tag_list.into_tags(),
            description: self.description,
            author: self.author.into_author(viewer),
            favorited: self.favorited,
            favorites_count: self.favorites_count,
        })
    }
}