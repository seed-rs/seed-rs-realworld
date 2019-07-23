use crate::entity::Viewer;

#[derive(Clone, Debug)]
pub enum Session {
    LoggedIn(Viewer),
    Guest,
}

impl<'a> Default for Session {
    fn default() -> Self {
        Session::Guest
    }
}

impl<'a> Session {
    pub fn new(viewer: Option<Viewer>) -> Self {
        match viewer {
            Some(viewer) => Session::LoggedIn(viewer),
            None => Session::Guest,
        }
    }
    pub fn viewer(&self) -> Option<&Viewer> {
        match self {
            Session::LoggedIn(viewer) => Some(viewer),
            Session::Guest => None,
        }
    }
}
