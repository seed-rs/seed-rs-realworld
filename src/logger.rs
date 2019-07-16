use std::borrow::Cow;

pub fn error(error: impl Into<Cow<'static, str>>) {
    error!("App error:", error.into())
}

pub fn errors(errors: impl IntoIterator<Item=impl Into<Cow<'static, str>>>) {
    for item in errors {
        error(item)
    }
}