#[derive(Clone, Default)]
pub struct PaginatedList<T> {
    pub values: Vec<T>,
    pub total: usize,
}