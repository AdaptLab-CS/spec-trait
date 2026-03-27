// Example 29 — WITH macros: Nested generic type specialization
// Specialize on Vec<Vec<u8>>, Vec<String>, etc. for flatten operations.
use spec_trait_macro::{spec, when};

pub struct Flattener;

pub trait FlattenTrait<T> {
    fn flat_len(&self, data: &[T]) -> usize;
}

impl<T> FlattenTrait<T> for Flattener {
    #[inline(never)]
    fn flat_len(&self, data: &[T]) -> usize {
        // Generic: just count outer elements
        data.len()
    }
}

#[when(T = Vec<Vec<u8>>)]
impl<T> FlattenTrait<T> for Flattener {
    #[inline(never)]
    fn flat_len(&self, data: &[T]) -> usize {
        // Nested bytes: count all inner bytes
        let mut total = 0usize;
        for outer in data {
            let vecs = unsafe { &*(outer as *const T as *const Vec<Vec<u8>>) };
            for inner in vecs {
                total += inner.len();
            }
        }
        total
    }
}

#[when(T = Vec<String>)]
impl<T> FlattenTrait<T> for Flattener {
    #[inline(never)]
    fn flat_len(&self, data: &[T]) -> usize {
        // Vec of strings: sum string lengths
        let mut total = 0usize;
        for outer in data {
            let strings = unsafe { &*(outer as *const T as *const Vec<String>) };
            for s in strings {
                total += s.len();
            }
        }
        total
    }
}

#[when(T = Vec<i32>)]
impl<T> FlattenTrait<T> for Flattener {
    #[inline(never)]
    fn flat_len(&self, data: &[T]) -> usize {
        // Vec<i32>: count i32 elements * 4 bytes
        let mut total = 0usize;
        for outer in data {
            let v = unsafe { &*(outer as *const T as *const Vec<i32>) };
            total += v.len() * 4;
        }
        total
    }
}

pub fn flat_len_vec_vec_u8(f: &Flattener, data: &[Vec<Vec<u8>>]) -> usize {
    spec! { f.flat_len(data); Flattener; [&[Vec<Vec<u8>>]] }
}

pub fn flat_len_vec_string(f: &Flattener, data: &[Vec<String>]) -> usize {
    spec! { f.flat_len(data); Flattener; [&[Vec<String>]] }
}

pub fn flat_len_vec_i32(f: &Flattener, data: &[Vec<i32>]) -> usize {
    spec! { f.flat_len(data); Flattener; [&[Vec<i32>]] }
}

pub fn flat_len_generic<T>(f: &Flattener, data: &[T]) -> usize {
    FlattenTrait::flat_len(f, data)
}
