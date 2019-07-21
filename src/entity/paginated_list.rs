use num_integer;

// @TODO encapsulate ; per_page has to be > 0
#[derive(Clone)]
pub struct PaginatedList<T> {
    pub values: Vec<T>,
    pub per_page: usize,
    pub total: usize,
}

impl<T> PaginatedList<T> {
    pub fn total_pages(&self) -> usize {
        num_integer::div_ceil(self.total, self.per_page)
    }
}

impl<T> Default for PaginatedList<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            per_page: 5,
            total: 0
        }
    }
}