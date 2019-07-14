use std::mem;

pub fn take<T: Default>(source: &mut T) -> T {
    mem::replace(source, T::default())
}