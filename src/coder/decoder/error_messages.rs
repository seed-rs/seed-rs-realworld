use crate::entity;
use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessages {
    errors: IndexMap<String, Vec<String>>,
}

impl ErrorMessages {
    pub fn into_error_messages(self) -> Vec<entity::ErrorMessage> {
        self.errors
            .into_iter()
            .map(|(field, errors)| format!("{} {}", field, errors.join(", ")).into())
            .collect()
    }
}

// ====== ====== TESTS ====== ======

// see `src/code/decoder/viewer` for example how to test decoder
