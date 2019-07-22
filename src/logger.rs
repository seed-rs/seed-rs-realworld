use std::fmt::Debug;

pub fn error(error: impl Debug) {
    error!("App error:", error)
}

pub fn errors(errors: impl IntoIterator<Item = impl Debug>) {
    for item in errors {
        error(item)
    }
}
