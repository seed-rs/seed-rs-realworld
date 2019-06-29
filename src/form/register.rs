use serde::Serialize;
use indexmap::{IndexSet, IndexMap};
use std::hash::{Hash, Hasher};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use unicode_segmentation::UnicodeSegmentation;
use crate::form::{self, FormField};

const MIN_PASSWORD_LENGTH: usize = 8;

pub type Form = form::Form<Field>;
pub type ValidForm = form::ValidForm<Field>;
pub type Problem = form::Problem;

impl Default for Form {
    fn default() -> Self {
        Self::new(Field::iter())
    }
}

#[derive(Clone, EnumIter)]
pub enum Field {
    Username(String),
    Email(String),
    Password(String)
}

impl FormField for Field {
    fn value(&self) -> &str {
        match self {
            Field::Username(value) => value,
            Field::Email(value) => value,
            Field::Password(value) => value,
        }
    }

    fn value_mut(&mut self) -> &mut String {
        match self {
            Field::Username(value) => value,
            Field::Email(value) => value,
            Field::Password(value) => value,
        }
    }

    fn key(&self) -> &'static str {
        match self {
            Field::Username(_) => "username",
            Field::Email(_) => "email",
            Field::Password(_) => "password",
        }
    }

    fn validate(&self) -> Option<form::Problem> {
        match self {
            Field::Username(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(self.key(), "username can't be blank."))
                } else {
                    None
                }
            },
            Field::Email(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(self.key(), "email can't be blank."))
                } else {
                    None
                }
            },
            Field::Password(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(
                        self.key(),
                        "password can't be blank."
                    ))
                } else if value.graphemes(true).count() < MIN_PASSWORD_LENGTH {
                    Some(form::Problem::new_invalid_field(
                        self.key(),
                        format!("password is too short (minimum is {} characters)", MIN_PASSWORD_LENGTH)
                    ))
                } else {
                    None
                }
            }
        }
    }
}
