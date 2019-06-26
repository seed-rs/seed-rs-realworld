use serde::Serialize;
use indexmap::IndexMap;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Field {
    Email,
    Password
}

impl Field {
    pub fn validate(&self, value: &str) -> Option<Problem> {
        match self {
            Field::Email => {
                if value.is_empty() {
                    Some(Problem::InvalidEntry(*self, "email can't be blank.".into()))
                } else {
                    None
                }
            },
            Field::Password => {
                if value.is_empty() {
                    Some(Problem::InvalidEntry(*self, "password can't be blank.".into()))
                } else {
                    None
                }
            }
        }
    }
}

pub enum Problem {
    InvalidEntry(Field, String),
    ServerError(String)
}


pub struct Form {
    pub user: IndexMap<Field, String>
}

impl Default for Form {
    fn default() -> Self {
        Self {
            user: vec![
                (Field::Email, "".to_string()),
                (Field::Password, "".to_string()),
            ].into_iter().collect()
        }
    }
}


impl Form {
    pub fn trim_fields(&self) -> TrimmedForm {
        TrimmedForm {
            user:
            self
                .user
                .iter()
                .map(|(field, value)|(field,value.trim()))
                .collect()
        }
    }
}

pub struct TrimmedForm<'a> {
    user: IndexMap<&'a Field, &'a str>
}

impl<'a> TrimmedForm<'a> {
    pub fn validate(&'a self) -> Result<ValidForm, Vec<Problem>> {
        let invalid_entries =
            self
                .user
                .iter()
                .filter_map(|(field,value)| {
                    field.validate(value)
                })
                .collect::<Vec<Problem>>();

        if invalid_entries.is_empty() {
            Ok(ValidForm {
                user:
                self.
                    user
                    .iter()
                    .map(|(field, value)| (**field, (*value).to_owned()))
                    .collect()
            })
        } else {
            Err(invalid_entries)
        }
    }
}

#[derive(Serialize)]
pub struct ValidForm {
    user: IndexMap<Field, String>
}