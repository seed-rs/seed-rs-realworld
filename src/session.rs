use crate::entity::Viewer;

#[derive(Clone, Debug)]
pub enum Session {
    LoggedIn(Viewer),
    Guest,
}

impl<'a> Default for Session {
    fn default() -> Self {
        Self::Guest
    }
}

impl<'a> Session {
    pub fn new(viewer: Option<Viewer>) -> Self {
        match viewer {
            Some(viewer) => Self::LoggedIn(viewer),
            None => Self::Guest,
        }
    }
    pub fn viewer(&self) -> Option<&Viewer> {
        match self {
            Self::LoggedIn(viewer) => Some(viewer),
            Self::Guest => None,
        }
    }
}
