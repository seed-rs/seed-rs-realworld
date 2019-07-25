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

// ====== ====== TESTS ====== ======

#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn total_pages_test() {
        // ====== ARRANGE ======
        let paginated_list: PaginatedList<()> = PaginatedList {
            per_page: NonZeroUsize::new(5).unwrap(),
            total: 6,
            ..PaginatedList::default()
        };

        // ====== ACT & ASSERT ======
        assert_eq!(paginated_list.total_pages(), 2);
    }
}
