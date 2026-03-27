// Example 31 — WITH macros: not() predicate – type exclusion
use spec_trait_macro::{spec, when};

pub struct Normalizer;

pub trait NormTrait<T> {
    fn compute(&self, data: &[T]) -> usize;
}

impl<T> NormTrait<T> for Normalizer {
    #[inline(never)]
    fn compute(&self, data: &[T]) -> usize {
        // Generic fallback
        data.len()
    }
}

#[when(T = String)]
impl<T> NormTrait<T> for Normalizer {
    #[inline(never)]
    fn compute(&self, data: &[T]) -> usize {
        // String: expensive heap-aware byte counting
        let mut total = 0usize;
        for s in data {
            total += s.len();
        }
        total
    }
}

#[when(not(T = String))]
impl<T> NormTrait<T> for Normalizer {
    #[inline(never)]
    fn compute(&self, data: &[T]) -> usize {
        // Non-String: fast size-based computation
        data.len() * std::mem::size_of::<T>()
    }
}

pub fn compute_i32(n: &Normalizer, data: &[i32]) -> usize {
    spec! { n.compute(data); Normalizer; [&[i32]] }
}

pub fn compute_u8(n: &Normalizer, data: &[u8]) -> usize {
    spec! { n.compute(data); Normalizer; [&[u8]] }
}

pub fn compute_string(n: &Normalizer, data: &[String]) -> usize {
    spec! { n.compute(data); Normalizer; [&[String]] }
}

pub fn compute_generic<T>(n: &Normalizer, data: &[T]) -> usize {
    NormTrait::compute(n, data)
}

#[inline(never)]
pub fn sum_compute_i32(n: &Normalizer, chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { n.compute(*chunk); Normalizer; [&[i32]] });
    }
    total
}

#[inline(never)]
pub fn sum_compute_generic<T>(n: &Normalizer, chunks: &[&[T]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(NormTrait::compute(n, *chunk));
    }
    total
}
