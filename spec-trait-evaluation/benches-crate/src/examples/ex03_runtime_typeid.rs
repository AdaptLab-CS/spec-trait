// Ex03 -- runtime TypeId dispatch: contains
use std::any::TypeId;

pub struct Searcher;

pub trait RtContainsTrait<T> {
    fn in_range(&self, item: &T, lo: &T, hi: &T) -> bool;
}

impl<T: PartialOrd + 'static> RtContainsTrait<T> for Searcher {
    #[inline(never)]
    fn in_range(&self, item: &T, lo: &T, hi: &T) -> bool {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let b = unsafe { *(item as *const T as *const u8) };
            let lo_b = unsafe { *(lo as *const T as *const u8) };
            let hi_b = unsafe { *(hi as *const T as *const u8) };
            b >= lo_b && b <= hi_b
        } else {
            item >= lo && item <= hi
        }
    }
}

pub fn contains_runtime(s: &Searcher, data: &[u8], lo: &u8, hi: &u8) -> usize {
    let mut count = 0;
    for item in data {
        if s.in_range(item, lo, hi) {
            count += 1;
        }
    }
    count
}
