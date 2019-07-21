use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::entity::form::{self, FormField};
use crate::coder::encoder::form::article_editor::ValidForm as ValidFormEncoder;

pub type Form = form::Form<Field>;
pub type ValidForm = form::ValidForm<Field>;
pub type Problem = form::Problem;

// ---- Form ----

impl Default for Form {
    fn default() -> Self {
        Self::new(Field::iter())
    }
}

impl ValidForm {
    pub fn to_encoder(&self) -> ValidFormEncoder {
        ValidFormEncoder::new(self)
    }
}

// ---- Field ----

#[derive(Clone, EnumIter)]
pub enum Field {
    Title(String),
    Description(String),
    Body(String),
    Tags(String),
}

impl FormField for Field {
    fn value(&self) -> &str {
        match self {
            Field::Title(value) => value,
            Field::Description(value) => value,
            Field::Body(value) => value,
            Field::Tags(value) => value,
        }
    }

    fn value_mut(&mut self) -> &mut String {
        match self {
            Field::Title(value) => value,
            Field::Description(value) => value,
            Field::Body(value) => value,
            Field::Tags(value) => value,
        }
    }

    fn key(&self) -> &'static str {
        match self {
            Field::Title(_) => "title",
            Field::Description(_) => "description",
            Field::Body(_) => "body",
            Field::Tags(_) => "tags",
        }
    }

    fn validate(&self) -> Option<form::Problem> {
        match self {
            Field::Title(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(self.key(), "title can't be blank"))
                } else {
                    None
                }
            },
            Field::Description(_) => None,
            Field::Body(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(self.key(), "body can't be blank"))
                } else {
                    None
                }
            },
            Field::Tags(_) => None,
        }
    }
}