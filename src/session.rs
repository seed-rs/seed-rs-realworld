use crate::viewer;

#[derive(Clone)]
pub enum Session {
    LoggedIn(viewer::Viewer),
    Guest
}

impl<'a> Default for Session {
    fn default() -> Self {
        Session::Guest
    }
}

impl<'a> Session {
    pub fn viewer(&self) -> Option<&viewer::Viewer> {
        match self {
            Session::LoggedIn(viewer) => Some(viewer),
            Session::Guest => None,
        }
    }
}

impl<'a> From<Option<viewer::Viewer>> for Session {
    fn from(viewer: Option<viewer::Viewer>) -> Session {
        match viewer {
            Some(viewer) => Session::LoggedIn(viewer),
            None => Session::default()
        }
    }
}