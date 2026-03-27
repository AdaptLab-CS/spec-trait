// Ex07 -- spec: Dedup (remove all duplicates, keep first occurrence)
use spec_trait_macro::{spec, when};
use std::collections::HashSet;
use std::hash::Hash;

pub struct Deduplicator;

pub trait DedupTrait<T> {
    fn dedup(&self, data: &[T]) -> Vec<T>;
}

impl<T: Clone + Eq + Hash> DedupTrait<T> for Deduplicator {
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

#[when(T = u8)]
impl<T> DedupTrait<T> for Deduplicator {
    #[inline(never)]
    fn dedup(&self, data: &[T]) -> Vec<T> {
        let mut seen = [false; 256];
        let mut out = Vec::new();
        for &b in data {
            if !seen[b as usize] {
                seen[b as usize] = true;
                out.push(b);
            }
        }
        out
    }
}

pub fn dedup_u8(d: &Deduplicator, data: &[u8]) -> Vec<u8> {
    spec! { d.dedup(data); Deduplicator; [&[u8]] }
}

pub fn _dedup_generic<T: Clone + Eq + Hash>(_d: &Deduplicator, data: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for item in data {
        if seen.insert(item) {
            out.push(item.clone());
        }
    }
    out
}
