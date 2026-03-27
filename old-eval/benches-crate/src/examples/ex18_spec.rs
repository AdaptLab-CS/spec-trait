// Example 18 — WITH macros: HRTB — higher-order function application
use spec_trait_macro::{spec, when};

pub struct Applicator;

pub trait ApplyTrait<F, T> {
    fn apply(&self, f: F, data: &[T]) -> Vec<T>;
}

impl<F: Fn(&T) -> T, T> ApplyTrait<F, T> for Applicator {
    #[inline(never)]
    fn apply(&self, f: F, data: &[T]) -> Vec<T> {
        data.iter().map(|x| f(x)).collect()
    }
}

#[when(F = for<'a> fn(&'a T) -> T)]
impl<F, T> ApplyTrait<F, T> for Applicator {
    #[inline(never)]
    fn apply(&self, f: F, data: &[T]) -> Vec<T> {
        // Specialized: fn pointer, can be called directly
        data.iter().map(|x| f(x)).collect()
    }
}

#[when(all(T = i32, F = for<'a> fn(&'a T) -> T))]
impl<F, T> ApplyTrait<F, T> for Applicator {
    #[inline(never)]
    fn apply(&self, f: F, data: &[T]) -> Vec<T> {
        // Most specialized: concrete i32 + fn pointer
        data.iter().map(|x| f(x)).collect()
    }
}

pub fn apply_i32_fn(a: &Applicator, f: fn(&i32) -> i32, data: &[i32]) -> Vec<i32> {
    spec! { a.apply(f, data); Applicator; [for<'a> fn(&'a i32) -> i32, &[i32]] }
}

pub fn apply_f64_fn(a: &Applicator, f: fn(&f64) -> f64, data: &[f64]) -> Vec<f64> {
    spec! { a.apply(f, data); Applicator; [for<'a> fn(&'a f64) -> f64, &[f64]] }
}

pub fn apply_generic<F: Fn(&i32) -> i32>(a: &Applicator, f: F, data: &[i32]) -> Vec<i32> {
    ApplyTrait::apply(a, f, data)
}
