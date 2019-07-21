use serde::Serialize;
use indexmap::IndexMap;
use crate::entity::form::{FormField, register::ValidForm as EntityValidForm};

#[derive(Serialize)]
pub struct ValidForm<'a> {
    user: IndexMap<&'a str, &'a str>
}

impl<'a> ValidForm<'a> {
    pub fn new(form: &'a EntityValidForm) -> Self {
        ValidForm {
            user: form
                .iter()
                .map(|(key, field)|(*key, field.value()))
                .collect()
        }
    }
}