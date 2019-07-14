#[derive(Clone)]
pub struct PaginatedList<T> {
    pub values: Vec<T>,
    pub total: usize,
}

impl<T> PaginatedList<T> {
    pub fn total_pages(&self) -> usize {
        self.total.checked_div(self.values.len())
            .unwrap_or_default()
    }
}

impl<T> Default for PaginatedList<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            total: 0
        }
    }
}