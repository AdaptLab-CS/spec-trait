// Example 32 — WITH macros: all() compound predicate with nested any()
use spec_trait_macro::{spec, when};

pub struct Folder;

pub trait FoldTrait<T> {
    fn fold_count(&self, data: &[T]) -> usize;
}

impl<T> FoldTrait<T> for Folder {
    #[inline(never)]
    fn fold_count(&self, data: &[T]) -> usize {
        // Generic fallback: plain count
        data.len()
    }
}

#[when(T: Copy)]
impl<T> FoldTrait<T> for Folder {
    #[inline(never)]
    fn fold_count(&self, data: &[T]) -> usize {
        // Copy: size-aware byte count
        data.len() * std::mem::size_of::<T>()
    }
}

#[when(all(T: Copy, any(T = i32, T = u8)))]
impl<T> FoldTrait<T> for Folder {
    #[inline(never)]
    fn fold_count(&self, data: &[T]) -> usize {
        // Copy + integer: byte count with extra pass
        let byte_count = data.len() * std::mem::size_of::<T>();
        byte_count.wrapping_add(data.len())
    }
}

#[when(T = i32)]
impl<T> FoldTrait<T> for Folder {
    #[inline(never)]
    fn fold_count(&self, data: &[T]) -> usize {
        // Most specific: i32 vectorized fold
        let mut acc = 0i64;
        for &x in data {
            acc = acc.wrapping_add(x as i64);
        }
        acc as usize
    }
}

pub fn fold_count_i32(f: &Folder, data: &[i32]) -> usize {
    spec! { f.fold_count(data); Folder; [&[i32]]; i32: Copy }
}

pub fn fold_count_u8(f: &Folder, data: &[u8]) -> usize {
    spec! { f.fold_count(data); Folder; [&[u8]]; u8: Copy }
}

pub fn fold_count_f64(f: &Folder, data: &[f64]) -> usize {
    spec! { f.fold_count(data); Folder; [&[f64]]; f64: Copy }
}

pub fn fold_count_generic<T>(f: &Folder, data: &[T]) -> usize {
    FoldTrait::fold_count(f, data)
}

#[inline(never)]
pub fn sum_fold_i32(f: &Folder, chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { f.fold_count(*chunk); Folder; [&[i32]]; i32: Copy });
    }
    total
}

#[inline(never)]
pub fn sum_fold_generic<T>(f: &Folder, chunks: &[&[T]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(FoldTrait::fold_count(f, *chunk));
    }
    total
}
