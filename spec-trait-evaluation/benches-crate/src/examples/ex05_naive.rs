// Ex05 -- naive: histogram — hash bucket (no specialization)
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HistogramOp;

pub trait NaiveBucketTrait<T> {
    fn bucket(&self, item: &T) -> usize;
}

impl<T: Hash> NaiveBucketTrait<T> for HistogramOp {
    #[inline(never)]
    fn bucket(&self, item: &T) -> usize {
        let mut h = DefaultHasher::new();
        item.hash(&mut h);
        (h.finish() % 256) as usize
    }
}

pub fn max_freq_naive(h: &HistogramOp, data: &[u8]) -> u32 {
    let mut counts = [0u32; 256];
    for item in data {
        counts[h.bucket(item)] += 1;
    }
    counts.iter().copied().max().unwrap_or(0)
}
