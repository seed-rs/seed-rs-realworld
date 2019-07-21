use crate::{viewer, api};

#[derive(Clone, Debug)]
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
    pub fn credentials(&self) -> Option<&api::Credentials> {
        self.viewer().map(|viewer| &viewer.credentials)
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