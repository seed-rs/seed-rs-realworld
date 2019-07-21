use crate::entity::{form::article_editor as form, author, timestamp, markdown, article};
use article::tag::IntoStrings;

pub mod feed;
pub mod slug;
pub mod tag;
pub mod comment;

#[derive(Clone)]
pub struct Article {
    pub title: String,
    pub slug: slug::Slug,
    pub body: markdown::Markdown,
    pub created_at: timestamp::Timestamp,
    pub updated_at: timestamp::Timestamp,
    pub tag_list: Vec<article::tag::Tag>,
    pub description: String,
    pub author: author::Author<'static>,
    pub favorited: bool,
    pub favorites_count: usize,
}

impl Article {
    pub fn into_form(self) -> form::Form {
        let fields: Vec<form::Field> = vec![
            form::Field::Title(self.title),
            form::Field::Description(self.description),
            form::Field::Body(self.body.to_string()),
            form::Field::Tags(self.tag_list.into_strings().join(" ")),
        ];
        form::Form::new(fields)
    }
}