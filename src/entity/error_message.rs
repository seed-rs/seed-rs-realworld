use shrinkwraprs::Shrinkwrap;
use std::borrow::Cow;

#[derive(Shrinkwrap, Clone, Debug)]
pub struct ErrorMessage(Cow<'static, str>);

impl ErrorMessage {
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_inner(self) -> Cow<'static, str> {
        self.0
    }
}

impl<T> From<T> for ErrorMessage
where
    T: Into<Cow<'static, str>>,
{
    fn from(error_message: T) -> Self {
        Self(error_message.into())
    }
}
