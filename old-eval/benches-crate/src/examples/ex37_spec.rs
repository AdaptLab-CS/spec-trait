// Example 37 — WITH macros: Generic receiver (Accumulator<T>) + type specialization
use spec_trait_macro::{spec, when};
use std::marker::PhantomData;

pub struct Accumulator<T>(pub PhantomData<T>);

pub trait AccumTrait<U> {
    fn accum_count(&self, data: &[U]) -> usize;
}

impl<T, U> AccumTrait<U> for Accumulator<T> {
    #[inline(never)]
    fn accum_count(&self, data: &[U]) -> usize {
        // Generic fallback
        data.len()
    }
}

#[when(U = i32)]
impl<T, U> AccumTrait<U> for Accumulator<T> {
    #[inline(never)]
    fn accum_count(&self, data: &[U]) -> usize {
        // i32: sum absolute values
        let mut acc = 0usize;
        for &x in data {
            acc = acc.wrapping_add((x as i64).unsigned_abs() as usize);
        }
        acc
    }
}

#[when(U = u8)]
impl<T, U> AccumTrait<U> for Accumulator<T> {
    #[inline(never)]
    fn accum_count(&self, data: &[U]) -> usize {
        // u8: saturating sum
        let mut acc = 0u64;
        for &b in data {
            acc = acc.saturating_add(b as u64);
        }
        acc as usize
    }
}

pub fn accum_i32(data: &[i32]) -> usize {
    let a: Accumulator<u8> = Accumulator(PhantomData);
    spec! { a.accum_count(data); Accumulator<u8>; [&[i32]] }
}

pub fn accum_u8(data: &[u8]) -> usize {
    let a: Accumulator<i32> = Accumulator(PhantomData);
    spec! { a.accum_count(data); Accumulator<i32>; [&[u8]] }
}

pub fn accum_generic<U>(data: &[U]) -> usize {
    let a: Accumulator<u8> = Accumulator(PhantomData);
    AccumTrait::accum_count(&a, data)
}

#[inline(never)]
pub fn sum_accum_i32(chunks: &[&[i32]]) -> usize {
    let a: Accumulator<u8> = Accumulator(PhantomData);
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { a.accum_count(*chunk); Accumulator<u8>; [&[i32]] });
    }
    total
}

#[inline(never)]
pub fn sum_accum_generic<U>(chunks: &[&[U]]) -> usize {
    let a: Accumulator<u8> = Accumulator(PhantomData);
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(AccumTrait::accum_count(&a, *chunk));
    }
    total
}
