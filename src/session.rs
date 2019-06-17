use crate::viewer;

pub enum Session<'a> {
    LoggedIn(viewer::Viewer<'a>),
    Guest
}

impl<'a> Default for Session<'a> {
    fn default() -> Self {
        Session::Guest
    }
}

impl<'a> Session<'a> {
    pub fn viewer(&self) -> Option<&viewer::Viewer> {
        match self {
            Session::LoggedIn(viewer) => Some(viewer),
            Session::Guest => None,
        }
    }
}

impl<'a> From<Option<viewer::Viewer<'a>>> for Session<'a> {
    fn from(viewer: Option<viewer::Viewer<'a>>) -> Session<'a> {
        match viewer {
            Some(viewer) => Session::LoggedIn(viewer),
            None => Session::default()
        }
    }
}