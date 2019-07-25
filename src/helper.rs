use std::mem;

pub fn take<T: Default>(source: &mut T) -> T {
    mem::replace(source, T::default())
}

// ====== ====== TESTS ====== ======

#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn take_test() {
        // ====== ARRANGE ======
        let mut text = "something".to_string();

        // ====== ACT ======
        let taken_text = take(&mut text);

        // ====== ASSERT ======
        assert_eq!(text, "");
        assert_eq!(taken_text, "something");
    }
}
