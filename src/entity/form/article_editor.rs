use crate::coder::encoder::form::article_editor::ValidForm as ValidFormEncoder;
use crate::entity::form::{self, FormField};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
        use Field::*;
        match self {
            Title(value) | Description(value) | Body(value) | Tags(value) => value,
        }
    }

    fn value_mut(&mut self) -> &mut String {
        use Field::*;
        match self {
            Title(value) | Description(value) | Body(value) | Tags(value) => value,
        }
    }

    fn key(&self) -> &'static str {
        use Field::*;
        match self {
            Title(_) => "title",
            Description(_) => "description",
            Body(_) => "body",
            Tags(_) => "tags",
        }
    }

    fn validate(&self) -> Option<form::Problem> {
        use Field::*;
        match self {
            Title(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(
                        self.key(),
                        "title can't be blank",
                    ))
                } else {
                    None
                }
            }
            Body(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(
                        self.key(),
                        "body can't be blank",
                    ))
                } else {
                    None
                }
            }
            Tags(_) | Description(_) => None,
        }
    }
}
