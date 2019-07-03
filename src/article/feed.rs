use crate::{session, paginated_list, article};

// Model

#[derive(Default)]
pub struct Model {
    session: session::Session,
    errors: Vec<String>,
    articles: paginated_list::PaginatedList<article::Article>,
    is_loading: bool,
}

// Init

pub fn init(
    session: session::Session,
    articles: paginated_list::PaginatedList<article::Article>
) -> Model {
    Model {
        session,
        articles,
        ..Model::default()
    }
}

// View

// Update