use serde::Serialize;
use indexmap::{IndexSet, IndexMap};
use std::hash::{Hash, Hasher};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::form::{self, FormField};

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
    pub fn dto(&self) -> ValidFormDTO {
        ValidFormDTO {
            user: self
                .0
                .iter()
                .map(|(key, field)|(*key, field.value()))
                .collect()
        }
    }
}

#[derive(Serialize)]
pub struct ValidFormDTO<'a> {
    user: IndexMap<&'a str, &'a str>
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
            Field::Description(value) => None,
            Field::Body(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(self.key(), "body can't be blank"))
                } else {
                    None
                }
            },
            Field::Tags(value) => None,
        }
    }
}