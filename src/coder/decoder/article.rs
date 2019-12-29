use crate::{
    coder::decoder,
    entity::{self, article::tag::IntoTags, ErrorMessage, Viewer},
};
use serde::Deserialize;
use std::{borrow::Cow, convert::TryInto};

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
    pub fn try_into_article(
        self,
        viewer: Option<Cow<Viewer>>,
    ) -> Result<entity::Article, ErrorMessage> {
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

// ====== ====== TESTS ====== ======

// see `src/code/decoder/viewer` for example how to test decoder
