// Ex04 -- runtime TypeId dispatch: sort
use std::any::TypeId;
use std::cmp::Ordering;

pub struct Sorter;

pub trait RtCmpTrait<T> {
    fn compare(&self, a: &T, b: &T) -> Ordering;
}

impl<T: Ord + 'static> RtCmpTrait<T> for Sorter {
    #[inline(never)]
    fn compare(&self, a: &T, b: &T) -> Ordering {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let a_byte = unsafe { *(a as *const T as *const u8) };
            let b_byte = unsafe { *(b as *const T as *const u8) };
            if a_byte < b_byte {
                Ordering::Less
            } else if a_byte > b_byte {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else {
            a.cmp(b)
        }
    }
}

pub fn sort_runtime(s: &Sorter, data: &mut [u8]) {
    data.sort_unstable_by(|a, b| s.compare(a, b));
}
