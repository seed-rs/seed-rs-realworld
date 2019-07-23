use num_integer::div_ceil;
use std::num::NonZeroUsize;

#[derive(Clone)]
pub struct PaginatedList<T> {
    pub items: Vec<T>,
    pub per_page: NonZeroUsize,
    pub total: usize,
}

impl<T> PaginatedList<T> {
    pub fn total_pages(&self) -> usize {
        div_ceil(self.total, self.per_page.get())
    }
}

impl<T> Default for PaginatedList<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            per_page: NonZeroUsize::new(5).unwrap(),
            total: 0,
        }
    }
}
