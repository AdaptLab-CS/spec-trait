// Example 30 — WITH macros: any() predicate – 4-way integer fast path
use spec_trait_macro::{spec, when};

pub struct Filter;

pub trait FilterTrait<T> {
    fn filter_count(&self, data: &[T]) -> usize;
}

impl<T> FilterTrait<T> for Filter {
    #[inline(never)]
    fn filter_count(&self, data: &[T]) -> usize {
        data.len()
    }
}

#[when(any(T = i32, T = u32, T = i64, T = u64))]
impl<T> FilterTrait<T> for Filter {
    #[inline(never)]
    fn filter_count(&self, data: &[T]) -> usize {
        // Integer fast path: byte-count via size_of
        data.len() * std::mem::size_of::<T>()
    }
}

#[when(T = u8)]
impl<T> FilterTrait<T> for Filter {
    #[inline(never)]
    fn filter_count(&self, data: &[T]) -> usize {
        // u8: count non-zero bytes
        let mut count = 0usize;
        for &b in data {
            count += (b != 0) as usize;
        }
        count
    }
}

#[when(T = f64)]
impl<T> FilterTrait<T> for Filter {
    #[inline(never)]
    fn filter_count(&self, data: &[T]) -> usize {
        // f64: magnitude-aware byte count
        data.len() * 8
    }
}

pub fn filter_count_i32(f: &Filter, data: &[i32]) -> usize {
    spec! { f.filter_count(data); Filter; [&[i32]] }
}

pub fn filter_count_u64(f: &Filter, data: &[u64]) -> usize {
    spec! { f.filter_count(data); Filter; [&[u64]] }
}

pub fn filter_count_u8(f: &Filter, data: &[u8]) -> usize {
    spec! { f.filter_count(data); Filter; [&[u8]] }
}

pub fn filter_count_f64(f: &Filter, data: &[f64]) -> usize {
    spec! { f.filter_count(data); Filter; [&[f64]] }
}

pub fn filter_count_generic<T>(f: &Filter, data: &[T]) -> usize {
    FilterTrait::filter_count(f, data)
}

#[inline(never)]
pub fn sum_filter_i32(f: &Filter, chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { f.filter_count(*chunk); Filter; [&[i32]] });
    }
    total
}

#[inline(never)]
pub fn sum_filter_generic<T>(f: &Filter, chunks: &[&[T]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(FilterTrait::filter_count(f, *chunk));
    }
    total
}
