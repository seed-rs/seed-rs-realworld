use crate::coder::encoder::form::login::ValidForm as ValidFormEncoder;
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
    Email(String),
    Password(String),
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
                    Some(form::Problem::new_invalid_field(
                        self.key(),
                        "email can't be blank",
                    ))
                } else {
                    None
                }
            }
            Field::Password(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(
                        self.key(),
                        "password can't be blank",
                    ))
                } else {
                    None
                }
            }
        }
    }
}
