// Example 15 — WITH macros: Contains / search specialization
use spec_trait_macro::{spec, when};

pub struct Searcher;

pub trait ContainsTrait<T> {
    fn contains(&self, haystack: &[T], needle: &T) -> bool;
}

impl<T: PartialEq> ContainsTrait<T> for Searcher {
    #[inline(never)]
    fn contains(&self, haystack: &[T], needle: &T) -> bool {
        haystack.iter().any(|x| x == needle)
    }
}

#[when(T = u8)]
impl<T> ContainsTrait<T> for Searcher {
    #[inline(never)]
    fn contains(&self, haystack: &[T], needle: &T) -> bool {
        memchr::memchr(*needle, haystack).is_some()
    }
}

#[when(T = i32)]
impl<T> ContainsTrait<T> for Searcher {
    #[inline(never)]
    fn contains(&self, haystack: &[T], needle: &T) -> bool {
        haystack.iter().any(|x| *x == *needle)
    }
}

pub fn contains_u8(s: &Searcher, haystack: &[u8], needle: &u8) -> bool {
    spec! { s.contains(haystack, needle); Searcher; [u8, u8] }
}

pub fn contains_i32(s: &Searcher, haystack: &[i32], needle: &i32) -> bool {
    spec! { s.contains(haystack, needle); Searcher; [i32, i32] }
}

pub fn contains_generic<T: PartialEq>(s: &Searcher, haystack: &[T], needle: &T) -> bool {
    ContainsTrait::contains(s, haystack, needle)
}
