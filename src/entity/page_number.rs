use newtype::NewType;

#[derive(NewType, Clone, Copy, PartialEq, Eq)]
pub struct PageNumber(usize);

impl Default for PageNumber {
    fn default() -> Self {
        Self(1)
    }
}
