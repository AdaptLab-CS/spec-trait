// Ex03 -- runtime TypeId dispatch: contains
use std::any::TypeId;

pub struct Searcher;

pub trait RtContainsTrait<T> {
    fn contains(&self, data: &[T], needle: &T) -> bool;
}

impl<T: PartialEq + 'static> RtContainsTrait<T> for Searcher {
    #[inline(never)]
    fn contains(&self, data: &[T], needle: &T) -> bool {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let byte_slice = unsafe {
                std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
            };
            let n = unsafe { *(needle as *const T as *const u8) };
            memchr::memchr(n, byte_slice).is_some()
        } else {
            data.iter().any(|x| x == needle)
        }
    }
}

pub fn contains_runtime(s: &Searcher, data: &[u8], needle: &u8) -> bool {
    s.contains(data, needle)
}
