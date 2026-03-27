// Ex06 -- naive: dedup — HashSet (no specialization)
use std::collections::HashSet;
use std::hash::Hash;

pub struct Deduplicator;

pub trait NaiveDedupTrait<T> {
    fn dedup(&self, data: &[T]) -> Vec<T>;
}

impl<T: Clone + Eq + Hash> NaiveDedupTrait<T> for Deduplicator {
    #[inline(never)]
    fn dedup(&self, data: &[T]) -> Vec<T> {
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for item in data {
            if seen.insert(item) {
                out.push(item.clone());
            }
        }
        out
    }
}

pub fn dedup_naive(d: &Deduplicator, data: &[u8]) -> Vec<u8> {
    d.dedup(data)
}
