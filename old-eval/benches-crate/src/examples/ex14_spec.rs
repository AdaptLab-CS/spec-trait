// Example 14 — WITH macros: FMA (fused multiply-add) specialization
use spec_trait_macro::{spec, when};

pub struct FmaComputer;

pub trait FmaTrait<T> {
    fn fma(&self, data: &[T], mul: T, add: T) -> Vec<T>;
}

impl<T: Clone> FmaTrait<T> for FmaComputer {
    #[inline(never)]
    fn fma(&self, data: &[T], _mul: T, _add: T) -> Vec<T> {
        data.to_vec() // generic fallback: identity
    }
}

#[when(T = f64)]
impl<T> FmaTrait<T> for FmaComputer {
    #[inline(never)]
    fn fma(&self, data: &[T], mul: T, add: T) -> Vec<T> {
        data.iter().map(|x| x.mul_add(mul, add)).collect()
    }
}

#[when(T = f32)]
impl<T> FmaTrait<T> for FmaComputer {
    #[inline(never)]
    fn fma(&self, data: &[T], mul: T, add: T) -> Vec<T> {
        data.iter().map(|x| x.mul_add(mul, add)).collect()
    }
}

#[when(T = i32)]
impl<T> FmaTrait<T> for FmaComputer {
    #[inline(never)]
    fn fma(&self, data: &[T], mul: T, add: T) -> Vec<T> {
        data.iter().map(|x| x.wrapping_mul(mul).wrapping_add(add)).collect()
    }
}

pub fn fma_f64(c: &FmaComputer, data: &[f64], mul: f64, add: f64) -> Vec<f64> {
    spec! { c.fma(data, mul, add); FmaComputer; [&[f64], f64, f64] }
}

pub fn fma_f32(c: &FmaComputer, data: &[f32], mul: f32, add: f32) -> Vec<f32> {
    spec! { c.fma(data, mul, add); FmaComputer; [&[f32], f32, f32] }
}

pub fn fma_i32(c: &FmaComputer, data: &[i32], mul: i32, add: i32) -> Vec<i32> {
    spec! { c.fma(data, mul, add); FmaComputer; [&[i32], i32, i32] }
}

pub fn fma_generic<T: Clone>(c: &FmaComputer, data: &[T], mul: T, add: T) -> Vec<T> {
    FmaTrait::fma(c, data, mul, add)
}
