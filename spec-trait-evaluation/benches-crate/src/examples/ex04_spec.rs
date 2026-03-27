// Ex04 — spec!: Sort with u8 counting-sort specialization
use spec_trait_macro::{spec, when};

pub struct Sorter;

pub trait SortTrait<T> {
    fn sort(&self, data: &mut [T]);
}

impl<T: Ord> SortTrait<T> for Sorter {
    #[inline(never)]
    fn sort(&self, data: &mut [T]) {
        data.sort_unstable();
    }
}

#[when(T = u8)]
impl<T> SortTrait<T> for Sorter {
    #[inline(never)]
    fn sort(&self, data: &mut [T]) {
        // Counting sort for u8 — O(n+256)
        let mut counts = [0u32; 256];
        for &b in data.iter() {
            counts[b as usize] += 1;
        }
        let mut idx = 0;
        for (val, &cnt) in counts.iter().enumerate() {
            for _ in 0..cnt {
                data[idx] = val as u8;
                idx += 1;
            }
        }
    }
}

pub fn sort_u8(s: &Sorter, data: &mut [u8]) {
    spec! { s.sort(data); Sorter; [&mut [u8]] }
}

pub fn sort_generic<T: Ord>(_s: &Sorter, data: &mut [T]) {
    data.sort_unstable();
}
