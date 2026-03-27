// Ex01 -- spec: Histogram -- max frequency count, specialized for u8
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
        let mut counts = [0u32; 256];
        for &b in data {
            counts[b as usize] += 1;
        }
        counts.iter().copied().max().unwrap_or(0)
    }
}

pub fn max_freq_u8(h: &HistogramOp, data: &[u8]) -> u32 {
    spec! { h.max_freq(data); HistogramOp; [&[u8]] }
}

pub fn _max_freq_generic<T: Eq + Hash>(_h: &HistogramOp, data: &[T]) -> u32 {
    let mut map = HashMap::<&T, u32>::new();
    for item in data {
        *map.entry(item).or_insert(0) += 1;
    }
    map.values().copied().max().unwrap_or(0)
}
