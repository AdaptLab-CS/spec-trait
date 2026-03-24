// Example 1 — WITH macros: Saturating byte doubling
// spec! resolves to the u8-specific impl at compile time.
use spec_trait_macro::{spec, when};

pub struct Doubler;

pub trait DoubleTrait<T> {
    fn double(&self, data: &[T]) -> Vec<T>;
}

impl<T: Clone> DoubleTrait<T> for Doubler {
    #[inline(never)]
    fn double(&self, data: &[T]) -> Vec<T> {
        data.to_vec()
    }
}

#[when(T = u8)]
impl<T> DoubleTrait<T> for Doubler {
    #[inline(never)]
    fn double(&self, data: &[T]) -> Vec<T> {
        data.iter().map(|b| b.saturating_mul(2)).collect()
    }
}

#[when(T = i32)]
impl<T> DoubleTrait<T> for Doubler {
    #[inline(never)]
    fn double(&self, data: &[T]) -> Vec<T> {
        data.iter().map(|v| v.wrapping_mul(2)).collect()
    }
}

pub fn double_u8(d: &Doubler, data: &[u8]) -> Vec<u8> {
    spec! { d.double(data); Doubler; [u8] }
}

pub fn double_i32(d: &Doubler, data: &[i32]) -> Vec<i32> {
    spec! { d.double(data); Doubler; [i32] }
}

pub fn double_generic<T: Clone>(d: &Doubler, data: &[T]) -> Vec<T> {
    DoubleTrait::double(d, data)
}
