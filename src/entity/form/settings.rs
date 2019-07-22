use crate::coder::encoder::form::settings::ValidForm as ValidFormEncoder;
use crate::entity::form::{self, FormField};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use unicode_segmentation::UnicodeSegmentation;

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
    Avatar(String),
    Username(String),
    Bio(String),
    Email(String),
    Password(String),
}

impl FormField for Field {
    fn value(&self) -> &str {
        match self {
            Field::Avatar(value) => value,
            Field::Username(value) => value,
            Field::Bio(value) => value,
            Field::Email(value) => value,
            Field::Password(value) => value,
        }
    }

    fn value_mut(&mut self) -> &mut String {
        match self {
            Field::Avatar(value) => value,
            Field::Username(value) => value,
            Field::Bio(value) => value,
            Field::Email(value) => value,
            Field::Password(value) => value,
        }
    }

    fn key(&self) -> &'static str {
        match self {
            Field::Avatar(_) => "image",
            Field::Username(_) => "username",
            Field::Bio(_) => "bio",
            Field::Email(_) => "email",
            Field::Password(_) => "password",
        }
    }

    fn validate(&self) -> Option<form::Problem> {
        match self {
            Field::Avatar(_) => None,
            Field::Username(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(
                        self.key(),
                        "username can't be blank",
                    ))
                } else {
                    None
                }
            }
            Field::Bio(_) => None,
            Field::Email(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(
                        self.key(),
                        "email can't be blank",
                    ))
                } else {
                    None
                }
            }
            Field::Password(value) => match value.graphemes(true).count() {
                1...form::MAX_INVALID_PASSWORD_LENGTH => Some(form::Problem::new_invalid_field(
                    self.key(),
                    format!(
                        "password is too short (minimum is {} characters)",
                        form::MIN_PASSWORD_LENGTH
                    ),
                )),
                _ => None,
            },
        }
    }
}
