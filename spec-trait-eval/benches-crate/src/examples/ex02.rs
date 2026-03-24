// Example 2 — WITH macros: Bitcount / popcount specialization
use spec_trait_macro::{spec, when};

pub struct Counter;

pub trait BitCount<T> {
    fn count_ones(&self, data: &[T]) -> u64;
}

impl<T> BitCount<T> for Counter {
    #[inline(never)]
    fn count_ones(&self, _data: &[T]) -> u64 {
        0 // generic fallback: no meaningful bit count
    }
}

#[when(T = u8)]
impl<T> BitCount<T> for Counter {
    #[inline(never)]
    fn count_ones(&self, data: &[T]) -> u64 {
        data.iter().map(|b| b.count_ones() as u64).sum()
    }
}

#[when(T = u32)]
impl<T> BitCount<T> for Counter {
    #[inline(never)]
    fn count_ones(&self, data: &[T]) -> u64 {
        data.iter().map(|v| v.count_ones() as u64).sum()
    }
}

#[when(T = u64)]
impl<T> BitCount<T> for Counter {
    #[inline(never)]
    fn count_ones(&self, data: &[T]) -> u64 {
        data.iter().map(|v| v.count_ones() as u64).sum()
    }
}

pub fn count_u8(c: &Counter, data: &[u8]) -> u64 {
    spec! { c.count_ones(data); Counter; [u8] }
}

pub fn count_u32(c: &Counter, data: &[u32]) -> u64 {
    spec! { c.count_ones(data); Counter; [u32] }
}

pub fn count_u64(c: &Counter, data: &[u64]) -> u64 {
    spec! { c.count_ones(data); Counter; [u64] }
}

pub fn count_generic<T>(c: &Counter, data: &[T]) -> u64 {
    BitCount::count_ones(c, data)
}
