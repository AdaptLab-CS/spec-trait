// Example 06 — WITH macros: Dedup specialization
use spec_trait_macro::{spec, when};

pub struct Deduplicator;

pub trait DedupTrait<T> {
    fn dedup(&self, data: &[T]) -> Vec<T>;
}

impl<T: Clone + PartialEq> DedupTrait<T> for Deduplicator {
    #[inline(never)]
    fn dedup(&self, data: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(data.len());
        for item in data {
            if out.last() != Some(item) {
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
        let mut out = Vec::with_capacity(data.len());
        let mut prev: Option<u8> = None;
        for &b in data {
            if prev != Some(b) {
                out.push(b);
                prev = Some(b);
            }
        }
        out
    }
}

#[when(T = i32)]
impl<T> DedupTrait<T> for Deduplicator {
    #[inline(never)]
    fn dedup(&self, data: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(data.len());
        let mut prev: Option<i32> = None;
        for &v in data {
            if prev != Some(v) {
                out.push(v);
                prev = Some(v);
            }
        }
        out
    }
}

pub fn dedup_u8(d: &Deduplicator, data: &[u8]) -> Vec<u8> {
    spec! { d.dedup(data); Deduplicator; [&[u8]] }
}

pub fn dedup_i32(d: &Deduplicator, data: &[i32]) -> Vec<i32> {
    spec! { d.dedup(data); Deduplicator; [&[i32]] }
}

pub fn dedup_generic<T: Clone + PartialEq>(d: &Deduplicator, data: &[T]) -> Vec<T> {
    DedupTrait::dedup(d, data)
}
