use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PageNumber(usize);

impl PageNumber {
    pub fn new(page_number: usize) -> Self {
        PageNumber(page_number)
    }
}

impl PageNumber {
    pub fn to_usize(&self) -> usize {
        self.0
    }
}

impl Default for PageNumber {
    fn default() -> Self {
        PageNumber(1)
    }
}

impl fmt::Display for PageNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
