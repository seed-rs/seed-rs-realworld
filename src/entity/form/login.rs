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
        use Field::*;
        match self {
            Email(value) | Password(value) => value,
        }
    }

    fn value_mut(&mut self) -> &mut String {
        use Field::*;
        match self {
            Email(value) | Password(value) => value,
        }
    }

    fn key(&self) -> &'static str {
        use Field::*;
        match self {
            Email(_) => "email",
            Password(_) => "password",
        }
    }

    fn validate(&self) -> Option<form::Problem> {
        use Field::*;
        match self {
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
            Password(value) => {
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
