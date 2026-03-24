// Example 9 — WITH macros: Copy vs Clone collection copy
use spec_trait_macro::{spec, when};

pub struct Copier;

pub trait CopyTrait<T> {
    fn copy_all(&self, data: &[T]) -> Vec<T>;
}

impl<T: Clone> CopyTrait<T> for Copier {
    #[inline(never)]
    fn copy_all(&self, data: &[T]) -> Vec<T> {
        data.to_vec() // Clone-based
    }
}

#[when(T: Copy)]
impl<T> CopyTrait<T> for Copier {
    #[inline(never)]
    fn copy_all(&self, data: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(data.len());
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), out.as_mut_ptr(), data.len());
            out.set_len(data.len());
        }
        out
    }
}

pub fn copy_i32(c: &Copier, data: &[i32]) -> Vec<i32> {
    spec! { c.copy_all(data); Copier; [i32]; i32: Copy }
}

pub fn copy_f64(c: &Copier, data: &[f64]) -> Vec<f64> {
    spec! { c.copy_all(data); Copier; [f64]; f64: Copy }
}

pub fn copy_u8(c: &Copier, data: &[u8]) -> Vec<u8> {
    spec! { c.copy_all(data); Copier; [u8]; u8: Copy }
}

pub fn copy_generic<T: Clone>(c: &Copier, data: &[T]) -> Vec<T> {
    CopyTrait::copy_all(c, data)
}
