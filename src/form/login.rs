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
    Email(String),
    Password(String)
}

impl FormField for Field {
    fn value(&self) -> &str {
        match self {
            Field::Email(value) => value,
            Field::Password(value) => value,
        }
    }

    fn value_mut(&mut self) -> &mut String {
        match self {
            Field::Email(value) => value,
            Field::Password(value) => value,
        }
    }

    fn key(&self) -> &'static str {
        match self {
            Field::Email(_) => "email",
            Field::Password(_) => "password",
        }
    }

    fn validate(&self) -> Option<form::Problem> {
        match self {
            Field::Email(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(self.key(), "email can't be blank."))
                } else {
                    None
                }
            },
            Field::Password(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(self.key(), "password can't be blank."))
                } else {
                    None
                }
            }
        }
    }
}