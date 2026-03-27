// Example 02 — WITH macros: Absolute value
use spec_trait_macro::{spec, when};

pub struct AbsComputer;

pub trait AbsTrait<T> {
    fn abs_all(&self, data: &[T]) -> Vec<T>;
}

impl<T: Clone> AbsTrait<T> for AbsComputer {
    #[inline(never)]
    fn abs_all(&self, data: &[T]) -> Vec<T> {
        data.to_vec() // generic: no-op copy
    }
}

#[when(T = i32)]
impl<T> AbsTrait<T> for AbsComputer {
    #[inline(never)]
    fn abs_all(&self, data: &[T]) -> Vec<T> {
        data.iter().map(|v| v.wrapping_abs()).collect()
    }
}

#[when(T = f64)]
impl<T> AbsTrait<T> for AbsComputer {
    #[inline(never)]
    fn abs_all(&self, data: &[T]) -> Vec<T> {
        data.iter().map(|v| v.abs()).collect()
    }
}

#[when(T = i64)]
impl<T> AbsTrait<T> for AbsComputer {
    #[inline(never)]
    fn abs_all(&self, data: &[T]) -> Vec<T> {
        data.iter().map(|v| v.wrapping_abs()).collect()
    }
}

pub fn abs_i32(c: &AbsComputer, data: &[i32]) -> Vec<i32> {
    spec! { c.abs_all(data); AbsComputer; [&[i32]] }
}

pub fn abs_f64(c: &AbsComputer, data: &[f64]) -> Vec<f64> {
    spec! { c.abs_all(data); AbsComputer; [&[f64]] }
}

pub fn abs_generic<T: Clone>(c: &AbsComputer, data: &[T]) -> Vec<T> {
    AbsTrait::abs_all(c, data)
}
