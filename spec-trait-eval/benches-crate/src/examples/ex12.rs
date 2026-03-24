// Example 12 — WITH macros: Sort specialization
use spec_trait_macro::{spec, when};

pub struct Sorter;

pub trait SortTrait<T> {
    fn sort(&self, data: &mut [T]);
}

impl<T: Ord> SortTrait<T> for Sorter {
    #[inline(never)]
    fn sort(&self, data: &mut [T]) {
        data.sort();
    }
}

#[when(T = u8)]
impl<T> SortTrait<T> for Sorter {
    #[inline(never)]
    fn sort(&self, data: &mut [T]) {
        // Counting sort for u8 — O(n)
        let mut counts = [0u32; 256];
        for &b in data.iter() {
            counts[b as usize] += 1;
        }
        let mut i = 0;
        for (val, &count) in counts.iter().enumerate() {
            for _ in 0..count {
                data[i] = val as u8;
                i += 1;
            }
        }
    }
}

#[when(T = i32)]
impl<T> SortTrait<T> for Sorter {
    #[inline(never)]
    fn sort(&self, data: &mut [T]) {
        data.sort_unstable();
    }
}

pub fn sort_u8(s: &Sorter, data: &mut [u8]) {
    spec! { s.sort(data); Sorter; [u8] }
}

pub fn sort_i32(s: &Sorter, data: &mut [i32]) {
    spec! { s.sort(data); Sorter; [i32] }
}

pub fn sort_generic<T: Ord>(s: &Sorter, data: &mut [T]) {
    SortTrait::sort(s, data)
}
