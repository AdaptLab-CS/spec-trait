// Example 30 — WITH macros: Combined trait bounds (T: Copy + Eq)
// Specialize on multiple trait bounds simultaneously for faster dedup.
use spec_trait_macro::{spec, when};

pub struct Deduper;

pub trait DedupCountTrait<T> {
    fn count_unique(&self, data: &[T]) -> usize;
}

impl<T: PartialEq> DedupCountTrait<T> for Deduper {
    #[inline(never)]
    fn count_unique(&self, data: &[T]) -> usize {
        // Generic PartialEq: O(n²) naive dedup count
        let mut count = 0usize;
        for i in 0..data.len() {
            let mut unique = true;
            for j in 0..i {
                if data[j] == data[i] {
                    unique = false;
                    break;
                }
            }
            if unique {
                count += 1;
            }
        }
        count
    }
}

#[when(T: Copy + Eq)]
impl<T> DedupCountTrait<T> for Deduper {
    #[inline(never)]
    fn count_unique(&self, data: &[T]) -> usize {
        // Copy + Eq: can use hash-free sorted approach since Copy is cheap
        let mut buf: Vec<T> = Vec::with_capacity(data.len());
        let mut count = 0usize;
        for &item in data {
            let found = buf.iter().any(|x| *x == item);
            if !found {
                buf.push(item);
                count += 1;
            }
        }
        count
    }
}

#[when(T = u8)]
impl<T> DedupCountTrait<T> for Deduper {
    #[inline(never)]
    fn count_unique(&self, data: &[T]) -> usize {
        // u8: use 256-element bitmap
        let mut seen = [false; 256];
        let mut count = 0usize;
        for &b in data {
            let idx = b as usize;
            if !seen[idx] {
                seen[idx] = true;
                count += 1;
            }
        }
        count
    }
}

pub fn dedup_u8(d: &Deduper, data: &[u8]) -> usize {
    spec! { d.count_unique(data); Deduper; [u8]; u8: Copy + Eq }
}

pub fn dedup_i32(d: &Deduper, data: &[i32]) -> usize {
    spec! { d.count_unique(data); Deduper; [i32]; i32: Copy + Eq }
}

pub fn dedup_generic<T: PartialEq>(d: &Deduper, data: &[T]) -> usize {
    DedupCountTrait::count_unique(d, data)
}

#[inline(never)]
pub fn sum_dedup_u8(d: &Deduper, chunks: &[&[u8]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { d.count_unique(*chunk); Deduper; [u8]; u8: Copy + Eq });
    }
    total
}

#[inline(never)]
pub fn sum_dedup_i32(d: &Deduper, chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { d.count_unique(*chunk); Deduper; [i32]; i32: Copy + Eq });
    }
    total
}

#[inline(never)]
pub fn sum_dedup_generic<T: PartialEq>(d: &Deduper, chunks: &[&[T]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(DedupCountTrait::count_unique(d, *chunk));
    }
    total
}
