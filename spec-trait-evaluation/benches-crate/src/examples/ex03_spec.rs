// Ex03 -- spec: contains — memchr SIMD vs linear scan
use spec_trait_macro::{spec, when};

pub struct Searcher;

pub trait ContainsTrait<T> {
    fn contains(&self, data: &[T], needle: &T) -> bool;
}

impl<T: PartialEq> ContainsTrait<T> for Searcher {
    #[inline(never)]
    fn contains(&self, data: &[T], needle: &T) -> bool {
        data.iter().any(|x| x == needle)
    }
}

#[when(T = u8)]
impl<T> ContainsTrait<T> for Searcher {
    #[inline(never)]
    fn contains(&self, data: &[T], needle: &T) -> bool {
        let byte_slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let n = unsafe { *(needle as *const T as *const u8) };
        memchr::memchr(n, byte_slice).is_some()
    }
}

pub fn contains_u8(s: &Searcher, data: &[u8], needle: &u8) -> bool {
    spec! { s.contains(data, needle); Searcher; [&[u8], &u8] }
}
