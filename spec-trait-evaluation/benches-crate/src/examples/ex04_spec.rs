// Ex04 -- spec: sort — counting sort for u8 vs generic sort_unstable
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
        let slice = unsafe {
            std::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u8, data.len())
        };
        let mut counts = [0u32; 256];
        for &b in slice.iter() {
            counts[b as usize] += 1;
        }
        let mut idx = 0;
        for (val, &cnt) in counts.iter().enumerate() {
            for _ in 0..cnt {
                slice[idx] = val as u8;
                idx += 1;
            }
        }
    }
}

pub fn sort_u8(s: &Sorter, data: &mut [u8]) {
    spec! { s.sort(data); Sorter; [&mut [u8]] }
}
