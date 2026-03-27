// Ex06 -- spec: dedup (keep first occurrence) — bitmap vs HashSet
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
        let slice = unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len())
        };
        let mut seen = [false; 256];
        let mut out_bytes = Vec::new();
        for &b in slice {
            if !seen[b as usize] {
                seen[b as usize] = true;
                out_bytes.push(b);
            }
        }
        let mut out = Vec::with_capacity(out_bytes.len());
        unsafe {
            std::ptr::copy_nonoverlapping(out_bytes.as_ptr(), out.as_mut_ptr() as *mut u8, out_bytes.len());
            out.set_len(out_bytes.len());
        }
        out
    }
}

pub fn dedup_u8(d: &Deduplicator, data: &[u8]) -> Vec<u8> {
    spec! { d.dedup(data); Deduplicator; [&[u8]] }
}
