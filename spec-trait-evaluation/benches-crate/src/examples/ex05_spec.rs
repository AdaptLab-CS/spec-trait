// Ex05 -- spec: histogram — array[256] indexing vs HashMap
use spec_trait_macro::{spec, when};
use std::collections::HashMap;
use std::hash::Hash;

pub struct HistogramOp;

pub trait HistogramTrait<T> {
    fn max_freq(&self, data: &[T]) -> u32;
}

impl<T: Eq + Hash> HistogramTrait<T> for HistogramOp {
    #[inline(never)]
    fn max_freq(&self, data: &[T]) -> u32 {
        let mut map = HashMap::<&T, u32>::new();
        for item in data {
            *map.entry(item).or_insert(0) += 1;
        }
        map.values().copied().max().unwrap_or(0)
    }
}

#[when(T = u8)]
impl<T> HistogramTrait<T> for HistogramOp {
    #[inline(never)]
    fn max_freq(&self, data: &[T]) -> u32 {
        let slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let mut counts = [0u32; 256];
        for &b in slice {
            counts[b as usize] += 1;
        }
        counts.iter().copied().max().unwrap_or(0)
    }
}

pub fn max_freq_u8(h: &HistogramOp, data: &[u8]) -> u32 {
    spec! { h.max_freq(data); HistogramOp; [&[u8]] }
}
