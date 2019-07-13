#[derive(Clone)]
pub struct PaginatedList<T> {
    pub values: Vec<T>,
    pub total: usize,
}

impl<T> Default for PaginatedList<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            total: 0
        }
    }
}