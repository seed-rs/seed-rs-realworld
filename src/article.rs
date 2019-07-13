use crate::{form::article_editor as form, username, author, timestamp};

pub mod feed;
pub mod slug;

#[derive(Clone)]
pub struct Article {
    pub title: String,
    pub slug: slug::Slug,
    pub body: String,
    pub created_at: timestamp::Timestamp,
    pub updated_at: timestamp::Timestamp,
    pub tag_list: Vec<String>,
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
            form::Field::Body(self.body),
            form::Field::Tags(self.tag_list.join(" ")),
        ];
        form::Form::new(fields)
    }
}