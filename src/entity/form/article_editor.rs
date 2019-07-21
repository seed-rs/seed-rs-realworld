use serde::Serialize;
use indexmap::IndexMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::entity::form::{self, FormField};

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
            article: self
                .0
                .iter()
                .map(|(key, field)|{
                    match field {
                        Field::Tags(tags) => {
                            ("tagList", ValidFormDTOValue::Vector(tags.split(" ").collect()))
                        }
                        _ => {
                            (*key, ValidFormDTOValue::Text(field.value()))
                        }
                    }
                })
                .collect()
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ValidFormDTOValue<'a> {
    Text(&'a str),
    Vector(Vec<&'a str>)
}

#[derive(Serialize)]
pub struct ValidFormDTO<'a> {
    article: IndexMap<&'a str, ValidFormDTOValue<'a>>
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
            Field::Description(_) => None,
            Field::Body(value) => {
                if value.is_empty() {
                    Some(form::Problem::new_invalid_field(self.key(), "body can't be blank"))
                } else {
                    None
                }
            },
            Field::Tags(_) => None,
        }
    }
}