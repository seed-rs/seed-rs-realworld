use std::borrow::Cow;

pub fn error(error: impl Into<Cow<'static, str>>) {
    error!("App error: '{}'", error.into())
}