use crate::viewer;

pub enum Session<'a> {
    LoggedIn(viewer::Viewer<'a>),
    Guest
}

impl<'a> From<Option<viewer::Viewer<'a>>> for Session<'a> {
    fn from(viewer: Option<viewer::Viewer<'a>>) -> Session<'a> {
        match viewer {
            Some(viewer) => Session::LoggedIn(viewer),
            None => Session::Guest
        }
    }
}