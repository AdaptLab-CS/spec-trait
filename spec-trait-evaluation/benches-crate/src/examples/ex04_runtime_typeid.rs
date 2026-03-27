// Ex04 -- runtime TypeId dispatch: sort
use std::any::TypeId;

pub struct Sorter;

pub trait RtSortTrait<T> {
    fn sort(&self, data: &mut [T]);
}

impl<T: Ord + 'static> RtSortTrait<T> for Sorter {
    #[inline(never)]
    fn sort(&self, data: &mut [T]) {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
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
        } else {
            data.sort_unstable();
        }
    }
}

pub fn sort_runtime(s: &Sorter, data: &mut [u8]) {
    s.sort(data);
}
