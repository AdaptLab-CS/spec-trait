// Example 10 — WITH macros: Min/Max specialization
use spec_trait_macro::{spec, when};

pub struct MinMaxer;

pub trait MinMaxTrait<T> {
    fn min_max(&self, data: &[T]) -> (T, T);
}

impl<T> MinMaxTrait<T> for MinMaxer {
    #[inline(never)]
    fn min_max(&self, _data: &[T]) -> (T, T) {
        // unimplemented!("generic fallback")

        // For simplicity, return a default value for the generic case.
        unsafe { std::mem::zeroed() }
    }
}

#[when(T = u8)]
impl<T> MinMaxTrait<T> for MinMaxer {
    #[inline(never)]
    fn min_max(&self, data: &[T]) -> (T, T) {
        let mut min = u8::MAX;
        let mut max = u8::MIN;
        for &v in data {
            if v < min { min = v; }
            if v > max { max = v; }
        }
        (min, max)
    }
}

#[when(T = i32)]
impl<T> MinMaxTrait<T> for MinMaxer {
    #[inline(never)]
    fn min_max(&self, data: &[T]) -> (T, T) {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &v in data {
            if v < min { min = v; }
            if v > max { max = v; }
        }
        (min, max)
    }
}

#[when(T = f64)]
impl<T> MinMaxTrait<T> for MinMaxer {
    #[inline(never)]
    fn min_max(&self, data: &[T]) -> (T, T) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for &v in data {
            if v < min { min = v; }
            if v > max { max = v; }
        }
        (min, max)
    }
}

pub fn minmax_u8(m: &MinMaxer, data: &[u8]) -> (u8, u8) {
    spec! { m.min_max(data); MinMaxer; [&[u8]] }
}

pub fn minmax_i32(m: &MinMaxer, data: &[i32]) -> (i32, i32) {
    spec! { m.min_max(data); MinMaxer; [&[i32]] }
}

pub fn minmax_f64(m: &MinMaxer, data: &[f64]) -> (f64, f64) {
    spec! { m.min_max(data); MinMaxer; [&[f64]] }
}
