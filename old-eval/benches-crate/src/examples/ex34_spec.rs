// Example 34 — WITH macros: 'static + Copy compound bound
use spec_trait_macro::{spec, when};

pub struct Cacher;

pub trait CacheTrait<T> {
    fn cached_size(&self, data: &[T]) -> usize;
}

impl<T> CacheTrait<T> for Cacher {
    #[inline(never)]
    fn cached_size(&self, data: &[T]) -> usize {
        // Generic fallback
        data.len()
    }
}

#[when(T: 'static)]
impl<T> CacheTrait<T> for Cacher {
    #[inline(never)]
    fn cached_size(&self, data: &[T]) -> usize {
        // 'static: size-aware count
        data.len() * std::mem::size_of::<T>()
    }
}

#[when(T: 'static + Copy)]
impl<T> CacheTrait<T> for Cacher {
    #[inline(never)]
    fn cached_size(&self, data: &[T]) -> usize {
        // 'static + Copy: cacheable, add header overhead
        data.len() * std::mem::size_of::<T>() + 8
    }
}

#[when(T = i32)]
impl<T> CacheTrait<T> for Cacher {
    #[inline(never)]
    fn cached_size(&self, data: &[T]) -> usize {
        // i32: most specific
        let mut acc = 0usize;
        for &x in data {
            acc = acc.wrapping_add((x as i64).unsigned_abs() as usize);
        }
        acc
    }
}

pub fn cached_size_i32(c: &Cacher, data: &[i32]) -> usize {
    spec! { c.cached_size(data); Cacher; [&[i32]]; i32: 'static + Copy }
}

pub fn cached_size_u8(c: &Cacher, data: &[u8]) -> usize {
    spec! { c.cached_size(data); Cacher; [&[u8]]; u8: 'static + Copy }
}

pub fn cached_size_string(c: &Cacher, data: &[String]) -> usize {
    spec! { c.cached_size(data); Cacher; [&[String]]; String: 'static }
}

pub fn cached_size_generic<T>(c: &Cacher, data: &[T]) -> usize {
    CacheTrait::cached_size(c, data)
}

#[inline(never)]
pub fn sum_cached_i32(c: &Cacher, chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { c.cached_size(*chunk); Cacher; [&[i32]]; i32: 'static + Copy });
    }
    total
}

#[inline(never)]
pub fn sum_cached_generic<T>(c: &Cacher, chunks: &[&[T]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(CacheTrait::cached_size(c, *chunk));
    }
    total
}
