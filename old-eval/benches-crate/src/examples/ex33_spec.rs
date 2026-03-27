// Example 33 — WITH macros: Multiple receiver types (3 receivers, same trait)
use spec_trait_macro::{spec, when};

pub struct VecCounter;
pub struct SliceCounter;
pub struct ArrayCounter;

pub trait CountTrait<T> {
    fn count(&self, data: &[T]) -> usize;
}

// --- VecCounter ---
impl<T> CountTrait<T> for VecCounter {
    #[inline(never)]
    fn count(&self, data: &[T]) -> usize {
        data.len()
    }
}

#[when(T = u8)]
impl<T> CountTrait<T> for VecCounter {
    #[inline(never)]
    fn count(&self, data: &[T]) -> usize {
        let mut c = 0usize;
        for &b in data {
            c += (b != 0) as usize;
        }
        c
    }
}

// --- SliceCounter ---
impl<T> CountTrait<T> for SliceCounter {
    #[inline(never)]
    fn count(&self, data: &[T]) -> usize {
        data.len() * 2
    }
}

#[when(T = u8)]
impl<T> CountTrait<T> for SliceCounter {
    #[inline(never)]
    fn count(&self, data: &[T]) -> usize {
        let mut total = 0usize;
        for &b in data {
            total = total.wrapping_add(b as usize);
        }
        total
    }
}

// --- ArrayCounter ---
impl<T> CountTrait<T> for ArrayCounter {
    #[inline(never)]
    fn count(&self, data: &[T]) -> usize {
        data.len() * 3
    }
}

#[when(T = u8)]
impl<T> CountTrait<T> for ArrayCounter {
    #[inline(never)]
    fn count(&self, data: &[T]) -> usize {
        let mut acc = 0u8;
        for &b in data {
            acc ^= b;
        }
        acc as usize
    }
}

pub fn count_vec_u8(vc: &VecCounter, data: &[u8]) -> usize {
    spec! { vc.count(data); VecCounter; [&[u8]] }
}

pub fn count_slice_u8(sc: &SliceCounter, data: &[u8]) -> usize {
    spec! { sc.count(data); SliceCounter; [&[u8]] }
}

pub fn count_array_u8(ac: &ArrayCounter, data: &[u8]) -> usize {
    spec! { ac.count(data); ArrayCounter; [&[u8]] }
}

pub fn count_vec_generic<T>(vc: &VecCounter, data: &[T]) -> usize {
    CountTrait::count(vc, data)
}

pub fn count_slice_generic<T>(sc: &SliceCounter, data: &[T]) -> usize {
    CountTrait::count(sc, data)
}

pub fn count_array_generic<T>(ac: &ArrayCounter, data: &[T]) -> usize {
    CountTrait::count(ac, data)
}

#[inline(never)]
pub fn sum_count_vec_u8(vc: &VecCounter, chunks: &[&[u8]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { vc.count(*chunk); VecCounter; [&[u8]] });
    }
    total
}

#[inline(never)]
pub fn sum_count_vec_generic<T>(vc: &VecCounter, chunks: &[&[T]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(CountTrait::count(vc, *chunk));
    }
    total
}
