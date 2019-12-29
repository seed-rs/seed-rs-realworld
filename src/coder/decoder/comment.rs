use crate::{
    coder::decoder,
    entity::{self, ErrorMessage, Viewer},
};
use serde::Deserialize;
use std::{borrow::Cow, convert::TryInto};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    id: usize,
    created_at: String,
    updated_at: String,
    body: String,
    author: decoder::Author,
}

impl Comment {
    pub fn try_into_comment(
        self,
        viewer: Option<Cow<Viewer>>,
    ) -> Result<entity::Comment, ErrorMessage> {
        let created_at = self.created_at.try_into()?;
        let updated_at = self.updated_at.try_into()?;

        Ok(entity::Comment {
            id: self.id.into(),
            body: self.body,
            created_at,
            updated_at,
            author: self.author.into_author(viewer),
        })
    }
}

// ====== ====== TESTS ====== ======

// see `src/code/decoder/viewer` for example how to test decoder
