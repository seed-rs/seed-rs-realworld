use crate::coder::encoder::form::settings::ValidForm as ValidFormEncoder;
use crate::entity::form::{self, FormField};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use unicode_segmentation::UnicodeSegmentation;

// ------ Form ------

pub type Form = form::Form<Field>;

impl Default for Form {
    fn default() -> Self {
        Self::new(Field::iter())
    }
}

// ------ ValidForm ------

pub type ValidForm = form::ValidForm<Field>;

impl ValidForm {
    pub fn to_encoder(&self) -> ValidFormEncoder {
        ValidFormEncoder::new(self)
    }
}

// ------ Problem ------

pub type Problem = form::Problem;

// ------ Field ------

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
        use Field::*;
        match self {
            Avatar(value) | Username(value) | Bio(value) | Email(value) | Password(value) => value,
        }
    }

    fn value_mut(&mut self) -> &mut String {
        use Field::*;
        match self {
            Avatar(value) | Username(value) | Bio(value) | Email(value) | Password(value) => value,
        }
    }

    fn key(&self) -> &'static str {
        use Field::*;
        match self {
            Avatar(_) => "image",
            Username(_) => "username",
            Bio(_) => "bio",
            Email(_) => "email",
            Password(_) => "password",
        }
    }

    fn validate(&self) -> Option<form::Problem> {
        use Field::*;
        match self {
            Avatar(_) | Bio(_) => None,
            Username(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(
                        self.key(),
                        "username can't be blank",
                    ))
                } else {
                    None
                }
            }
            Email(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(
                        self.key(),
                        "email can't be blank",
                    ))
                } else {
                    None
                }
            }
            Password(value) => match value.graphemes(true).count() {
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
