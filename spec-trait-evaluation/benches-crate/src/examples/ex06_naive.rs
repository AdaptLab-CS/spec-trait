// Ex06 -- naive: transform — hash-based remap (no specialization)
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct Deduplicator;

pub trait NaiveTransformTrait<T> {
    fn transform(&self, item: &T) -> u8;
}

impl<T: Hash> NaiveTransformTrait<T> for Deduplicator {
    #[inline(never)]
    fn transform(&self, item: &T) -> u8 {
        let mut h = DefaultHasher::new();
        item.hash(&mut h);
        (h.finish() & 0xFF) as u8
    }
}

pub fn dedup_naive(d: &Deduplicator, data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());
    for item in data {
        result.push(d.transform(item));
    }
    result
}
