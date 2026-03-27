// Ex05 -- naive: histogram — HashMap (no specialization)
use std::collections::HashMap;
use std::hash::Hash;

pub struct HistogramOp;

pub trait NaiveHistogramTrait<T> {
    fn max_freq(&self, data: &[T]) -> u32;
}

impl<T: Eq + Hash> NaiveHistogramTrait<T> for HistogramOp {
    #[inline(never)]
    fn max_freq(&self, data: &[T]) -> u32 {
        let mut map = HashMap::<&T, u32>::new();
        for item in data {
            *map.entry(item).or_insert(0) += 1;
        }
        map.values().copied().max().unwrap_or(0)
    }
}

pub fn max_freq_naive(h: &HistogramOp, data: &[u8]) -> u32 {
    h.max_freq(data)
}
