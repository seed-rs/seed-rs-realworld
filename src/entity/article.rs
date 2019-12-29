use crate::entity::{
    article::tag::IntoStrings,
    form::article_editor::{Field, Form},
    Author, Markdown, Tag, Timestamp,
};
use slug::Slug;

pub mod comment;
pub mod feed;
pub mod slug;
pub mod tag;

#[derive(Clone)]
pub struct Article {
    pub title: String,
    pub slug: Slug,
    pub body: Markdown,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub tag_list: Vec<Tag>,
    pub description: String,
    pub author: Author,
    pub favorited: bool,
    pub favorites_count: usize,
}

impl Article {
    pub fn into_form(self) -> Form {
        Form::new(vec![
            Field::Title(self.title),
            Field::Description(self.description),
            Field::Body(self.body.to_string()),
            Field::Tags(self.tag_list.into_strings().join(" ")),
        ])
    }
}
