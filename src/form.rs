use serde::Serialize;
use indexmap::IndexMap;
use std::hash::{Hash, Hasher};
use strum_macros::EnumIter;
use std::borrow::Cow;

pub mod login;
pub mod register;
pub mod settings;
pub mod article_editor;

const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_INVALID_PASSWORD_LENGTH: usize = MIN_PASSWORD_LENGTH - 1;

// ----- Field -----

type FieldKey = &'static str;

pub trait FormField: Clone {
    fn value(&self) -> &str;
    fn value_mut(&mut self) -> &mut String;
    fn key(&self) -> &'static str;
    fn validate(&self) -> Option<Problem>;
}

// ----- Problem -----

#[derive(Clone)]
pub enum Problem {
    InvalidField { field_key: &'static str, message: Cow<'static, str> },
    ServerError { message: Cow<'static, str> }
}

impl Problem {
    pub fn new_invalid_field(field_key: &'static str, message: impl Into<Cow<'static, str>>) -> Self {
        Problem::InvalidField { field_key, message: message.into() }
    }
    pub fn new_server_error(message: impl Into<Cow<'static, str>>) -> Self {
        Problem::ServerError { message: message.into() }
    }
    pub fn message(&self) -> &str {
        match self {
            Problem::InvalidField { message, ..} => message,
            Problem::ServerError { message} => message,
        }
    }
}

// ----- Form -----

pub struct Form<T: FormField>(IndexMap<FieldKey, T>);

impl<T: FormField> Form<T> {
    pub fn new(fields: impl IntoIterator<Item=T>) -> Self {
        Self(fields.into_iter().map(|field|(field.key(), field)).collect())
    }

    pub fn trim_fields(&self) -> TrimmedForm<T> {
        TrimmedForm (
            self
                .0
                .iter()
                .map(|(key, field)| {
                    let mut field = field.clone();
                    let mut value = field.value_mut();
                    *value = value.trim().into();
                    (*key, field)
                } )
                .collect()
        )
    }

    pub fn iter(&self) -> indexmap::map::Values<FieldKey, T> {
        self.0.values()
    }

    pub fn upsert_field(&mut self, field: T) {
        self.0.insert(field.key(), field);
    }
}

// ----- TrimmedForm -----

pub struct TrimmedForm<T: FormField>(IndexMap<FieldKey, T>);

impl<T: FormField> TrimmedForm<T> {
    pub fn validate(self) -> Result<ValidForm<T>, Vec<Problem>> {
        let invalid_entries =
            self
                .0
                .iter()
                .filter_map(|(_, field)|field.validate())
                .collect::<Vec<Problem>>();

        if invalid_entries.is_empty() {
            Ok(ValidForm(self.0) )
        } else {
            Err(invalid_entries)
        }
    }
}

// ----- ValidForm -----

pub struct ValidForm<T: FormField>(IndexMap<FieldKey, T>);