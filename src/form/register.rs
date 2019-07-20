use serde::Serialize;
use indexmap::IndexMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use unicode_segmentation::UnicodeSegmentation;
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
                    Some(form::Problem::new_invalid_field(self.key(), "username can't be blank"))
                } else {
                    None
                }
            },
            Field::Email(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(self.key(), "email can't be blank"))
                } else {
                    None
                }
            },
            Field::Password(value) => {
                match value.graphemes(true).count() {
                    0 => {
                        Some(form::Problem::new_invalid_field(
                            self.key(),
                            "password can't be blank"
                        ))
                    }
                    1...form::MAX_INVALID_PASSWORD_LENGTH => {
                        Some(form::Problem::new_invalid_field(
                            self.key(),
                            format!(
                                "password is too short (minimum is {} characters)",
                                form::MIN_PASSWORD_LENGTH
                            )
                        ))
                    }
                    _ => None
                }
            }
        }
    }
}
