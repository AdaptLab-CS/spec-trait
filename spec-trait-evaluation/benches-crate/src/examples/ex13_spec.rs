// Ex13 -- spec: flatten_sum — compound all() cross-parameter specialization
//
// Pattern: #[when(all(U = Vec<T>, T = u8))] and #[when(U = Vec<T>)]
// These compound conditions relating two type parameters cannot be expressed with
// enum dispatch, TypeId dispatch, or naive generics.
// Only spec-trait can express "U is Vec<T> AND T is u8" as a single dispatch condition.
use spec_trait_macro::{spec, when};

pub struct Flattener;

pub trait FlattenSumTrait<T, U> {
    fn flatten_sum(&self, outer: &[U]) -> u64;
}

impl<T, U> FlattenSumTrait<T, U> for Flattener {
    #[inline(never)]
    fn flatten_sum(&self, _outer: &[U]) -> u64 {
        // Default: cannot flatten unknown U, return 0
        0
    }
}

#[when(all(U = Vec<T>, T = u8))]
impl<T, U> FlattenSumTrait<T, U> for Flattener {
    #[inline(never)]
    fn flatten_sum(&self, outer: &[U]) -> u64 {
        // Specialized: U = Vec<u8>, T = u8 — direct byte-level sum
        let vecs = unsafe {
            std::slice::from_raw_parts(outer.as_ptr() as *const Vec<u8>, outer.len())
        };
        let mut acc: u64 = 0;
        for v in vecs {
            for &b in v.iter() {
                acc += b as u64;
            }
        }
        acc
    }
}

#[when(U = Vec<T>)]
impl<T: Into<u64> + Copy, U> FlattenSumTrait<T, U> for Flattener {
    #[inline(never)]
    fn flatten_sum(&self, outer: &[U]) -> u64 {
        // Specialized: U = Vec<T> (generic T) — flatten and sum
        let vecs = unsafe {
            std::slice::from_raw_parts(outer.as_ptr() as *const Vec<T>, outer.len())
        };
        let mut acc: u64 = 0;
        for v in vecs {
            for &item in v.iter() {
                acc += item.into();
            }
        }
        acc
    }
}

pub fn flatten_sum_vec_u8(f: &Flattener, outer: &[Vec<u8>]) -> u64 {
    spec! { f.flatten_sum(outer); Flattener; [&[Vec<u8>]] }
}
