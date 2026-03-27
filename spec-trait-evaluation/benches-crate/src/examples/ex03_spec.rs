// Ex03 — spec!: Contains search, u8 specialized via memchr
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
        memchr::memchr(*needle, data).is_some()
    }
}

pub fn contains_u8(s: &Searcher, data: &[u8], needle: &u8) -> bool {
    spec! { s.contains(data, needle); Searcher; [&[u8], &u8] }
}

pub fn contains_generic<T: PartialEq>(_s: &Searcher, data: &[T], needle: &T) -> bool {
    data.iter().any(|x| x == needle)
}
