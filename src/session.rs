use crate::entity::Viewer;

#[derive(Clone, Debug)]
pub enum Session {
    LoggedIn(Viewer),
    Guest
}

impl<'a> Default for Session {
    fn default() -> Self {
        Session::Guest
    }
}

impl<'a> Session {
    pub fn viewer(&self) -> Option<&Viewer> {
        match self {
            Session::LoggedIn(viewer) => Some(viewer),
            Session::Guest => None,
        }
    }
}

impl<'a> From<Option<Viewer>> for Session {
    fn from(viewer: Option<Viewer>) -> Session {
        match viewer {
            Some(viewer) => Session::LoggedIn(viewer),
            None => Session::default()
        }
    }
}