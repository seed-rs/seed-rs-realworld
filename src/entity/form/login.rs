use crate::{
    coder::encoder::form::login::ValidForm as ValidFormEncoder,
    entity::form::{self, FormField},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

// ------ Form ------

pub type Problem = form::Problem;

// ------ Field ------

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

// ====== ====== TESTS ====== ======

#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn valid_form_test() {
        // ====== ARRANGE ======
        let mut form = Form::default();
        form.upsert_field(Field::Email("john@example.com".into()));
        form.upsert_field(Field::Password("John's password".into()));

        // ====== ACT ======
        let result = form.trim_fields().validate();

        // ====== ASSERT ======
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn invalid_form_test() {
        // ====== ARRANGE ======
        let form = Form::default();

        // ====== ACT ======
        let result = form.trim_fields().validate();

        // ====== ASSERT ======
        assert!(if let Err(problems) = result {
            vec!["email can't be blank", "password can't be blank"]
                == problems
                    .iter()
                    .map(form::Problem::message)
                    .collect::<Vec<_>>()
        } else {
            false
        });
    }
}
