use crate::viewer;

pub enum Session {
    LoggedIn(viewer::Viewer),
    Guest
}

impl From<Option<viewer::Viewer>> for Session {
    fn from(viewer: Option<viewer::Viewer>) -> Session {
        match viewer {
            Some(viewer) => Session::LoggedIn(viewer),
            None => Session::Guest
        }
    }
}