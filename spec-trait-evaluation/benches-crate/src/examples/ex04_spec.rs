// Ex04 -- spec: sort — direct byte comparison for u8 vs generic Ord
use spec_trait_macro::{spec, when};
use std::cmp::Ordering;

pub struct Sorter;

pub trait CmpTrait<T> {
    fn compare(&self, a: &T, b: &T) -> Ordering;
}

impl<T: Ord> CmpTrait<T> for Sorter {
    #[inline(never)]
    fn compare(&self, a: &T, b: &T) -> Ordering {
        a.cmp(b)
    }
}

#[when(T = u8)]
impl<T> CmpTrait<T> for Sorter {
    #[inline(never)]
    fn compare(&self, a: &T, b: &T) -> Ordering {
        let a_byte = unsafe { *(a as *const T as *const u8) };
        let b_byte = unsafe { *(b as *const T as *const u8) };
        if a_byte < b_byte {
            Ordering::Less
        } else if a_byte > b_byte {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[inline(always)]
fn compare_spec(s: &Sorter, a: &u8, b: &u8) -> Ordering {
    spec! { s.compare(a, b); Sorter; [&u8, &u8] }
}

pub fn sort_u8(s: &Sorter, data: &mut [u8]) {
    data.sort_unstable_by(|a, b| compare_spec(s, a, b));
}
