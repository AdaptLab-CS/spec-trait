// Ex05 -- runtime TypeId dispatch: histogram
use std::any::TypeId;
use std::collections::HashMap;
use std::hash::Hash;

pub struct HistogramOp;

pub trait RtHistogramTrait<T> {
    fn max_freq(&self, data: &[T]) -> u32;
}

impl<T: Eq + Hash + 'static> RtHistogramTrait<T> for HistogramOp {
    #[inline(never)]
    fn max_freq(&self, data: &[T]) -> u32 {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let slice = unsafe {
                std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
            };
            let mut counts = [0u32; 256];
            for &b in slice {
                counts[b as usize] += 1;
            }
            counts.iter().copied().max().unwrap_or(0)
        } else {
            let mut map = HashMap::<&T, u32>::new();
            for item in data {
                *map.entry(item).or_insert(0) += 1;
            }
            map.values().copied().max().unwrap_or(0)
        }
    }
}

pub fn max_freq_runtime(h: &HistogramOp, data: &[u8]) -> u32 {
    h.max_freq(data)
}
