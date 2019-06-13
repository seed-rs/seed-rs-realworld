use crate::viewer;

#[derive(Clone)]
pub enum Session {
    LoggedIn(viewer::Viewer),
    Guest
}

impl From<Option<viewer::Viewer>> for Session {
    fn from(viewer: Option<viewer::Viewer>) -> Session {
        if let Some(viewer) = viewer {
            Session::LoggedIn(viewer)
        } else {
            Session::Guest
        }
    }
}