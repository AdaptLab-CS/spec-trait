// Example 3 — WITH macros: Sum specialization
use spec_trait_macro::{spec, when};

pub struct Summator;

pub trait SumTrait<T> {
    fn sum(&self, data: &[T]) -> T;
}

impl<T> SumTrait<T> for Summator {
    #[inline(never)]
    fn sum(&self, _data: &[T]) -> T {
        unimplemented!("generic fallback")
    }
}

#[when(T = i32)]
impl<T> SumTrait<T> for Summator {
    #[inline(never)]
    fn sum(&self, data: &[T]) -> T {
        data.iter().copied().sum::<i32>()
    }
}

#[when(T = f64)]
impl<T> SumTrait<T> for Summator {
    #[inline(never)]
    fn sum(&self, data: &[T]) -> T {
        data.iter().copied().sum::<f64>()
    }
}

#[when(T = u8)]
impl<T> SumTrait<T> for Summator {
    #[inline(never)]
    fn sum(&self, data: &[T]) -> T {
        let mut acc: u8 = 0;
        for &b in data {
            acc = acc.wrapping_add(b);
        }
        acc
    }
}

pub fn sum_i32(s: &Summator, data: &[i32]) -> i32 {
    spec! { s.sum(data); Summator; [i32] }
}

pub fn sum_f64(s: &Summator, data: &[f64]) -> f64 {
    spec! { s.sum(data); Summator; [f64] }
}

pub fn sum_u8(s: &Summator, data: &[u8]) -> u8 {
    spec! { s.sum(data); Summator; [u8] }
}
