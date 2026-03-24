// Example 20 — WITH macros: Dot product specialization
use spec_trait_macro::{spec, when};

pub struct DotProduct;

pub trait DotTrait<T> {
    fn dot(&self, a: &[T], b: &[T]) -> T;
}

impl<T> DotTrait<T> for DotProduct {
    #[inline(never)]
    fn dot(&self, _a: &[T], _b: &[T]) -> T {
        unimplemented!("generic fallback")
    }
}

#[when(T = f64)]
impl<T> DotTrait<T> for DotProduct {
    #[inline(never)]
    fn dot(&self, a: &[T], b: &[T]) -> T {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f64>()
    }
}

#[when(T = f32)]
impl<T> DotTrait<T> for DotProduct {
    #[inline(never)]
    fn dot(&self, a: &[T], b: &[T]) -> T {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>()
    }
}

#[when(T = i32)]
impl<T> DotTrait<T> for DotProduct {
    #[inline(never)]
    fn dot(&self, a: &[T], b: &[T]) -> T {
        a.iter().zip(b.iter()).map(|(x, y)| x.wrapping_mul(*y)).sum::<i32>()
    }
}

pub fn dot_f64(d: &DotProduct, a: &[f64], b: &[f64]) -> f64 {
    spec! { d.dot(a, b); DotProduct; [f64, f64] }
}

pub fn dot_f32(d: &DotProduct, a: &[f32], b: &[f32]) -> f32 {
    spec! { d.dot(a, b); DotProduct; [f32, f32] }
}

pub fn dot_i32(d: &DotProduct, a: &[i32], b: &[i32]) -> i32 {
    spec! { d.dot(a, b); DotProduct; [i32, i32] }
}
