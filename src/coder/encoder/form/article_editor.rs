use crate::entity::form::{
    article_editor::{Field, ValidForm as EntityValidForm},
    FormField,
};
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct ValidForm<'a> {
    article: IndexMap<&'a str, ValidFormValue<'a>>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum ValidFormValue<'a> {
    Text(&'a str),
    TextList(Vec<&'a str>),
}

impl<'a> ValidForm<'a> {
    pub fn new(form: &'a EntityValidForm) -> Self {
        ValidForm {
            article: form
                .iter_keys_and_fields()
                .map(|(key, field)| match field {
                    Field::Tags(tags) => (
                        "tagList",
                        ValidFormValue::TextList(tags.split(' ').collect()),
                    ),
                    _ => (*key, ValidFormValue::Text(field.value())),
                })
                .collect(),
        }
    }
}

// ====== ====== TESTS ====== ======

// see `src/code/encoder/comments` for example how to test encoder
