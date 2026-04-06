// Ex03 -- spec: contains — byte range check vs PartialOrd
use spec_trait_macro::{spec, when};

pub struct Searcher;

pub trait ContainsTrait<T> {
    fn in_range(&self, item: &T, lo: &T, hi: &T) -> bool;
}

impl<T: PartialOrd> ContainsTrait<T> for Searcher {
    #[inline(never)]
    fn in_range(&self, item: &T, lo: &T, hi: &T) -> bool {
        item >= lo && item <= hi
    }
}

#[when(T = u8)]
impl<T> ContainsTrait<T> for Searcher {
    #[inline(never)]
    fn in_range(&self, item: &T, lo: &T, hi: &T) -> bool {
        let b = unsafe { *(item as *const T as *const u8) };
        let lo_b = unsafe { *(lo as *const T as *const u8) };
        let hi_b = unsafe { *(hi as *const T as *const u8) };
        b >= lo_b && b <= hi_b
    }
}

#[inline(always)]
fn in_range_spec(s: &Searcher, item: &u8, lo: &u8, hi: &u8) -> bool {
    spec! { s.in_range(item, lo, hi); Searcher; [&u8, &u8, &u8] }
}

pub fn contains_u8(s: &Searcher, data: &[u8], lo: &u8, hi: &u8) -> usize {
    let mut count = 0;
    for item in data {
        if in_range_spec(s, item, lo, hi) {
            count += 1;
        }
    }
    count
}
