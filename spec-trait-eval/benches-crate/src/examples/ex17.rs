// Example 17 — WITH macros: Numeric comparison / clamping
use spec_trait_macro::{spec, when};

pub struct Clamper;

pub trait ClampTrait<T> {
    fn clamp_all(&self, data: &[T], lo: T, hi: T) -> Vec<T>;
}

impl<T: Clone + PartialOrd> ClampTrait<T> for Clamper {
    #[inline(never)]
    fn clamp_all(&self, data: &[T], lo: T, hi: T) -> Vec<T> {
        data.iter().map(|x| {
            if *x < lo { lo.clone() }
            else if *x > hi { hi.clone() }
            else { x.clone() }
        }).collect()
    }
}

#[when(T = i32)]
impl<T> ClampTrait<T> for Clamper {
    #[inline(never)]
    fn clamp_all(&self, data: &[T], lo: T, hi: T) -> Vec<T> {
        data.iter().map(|&x| x.clamp(lo, hi)).collect()
    }
}

#[when(T = f64)]
impl<T> ClampTrait<T> for Clamper {
    #[inline(never)]
    fn clamp_all(&self, data: &[T], lo: T, hi: T) -> Vec<T> {
        data.iter().map(|&x| x.clamp(lo, hi)).collect()
    }
}

#[when(T = u8)]
impl<T> ClampTrait<T> for Clamper {
    #[inline(never)]
    fn clamp_all(&self, data: &[T], lo: T, hi: T) -> Vec<T> {
        data.iter().map(|&x| x.clamp(lo, hi)).collect()
    }
}

pub fn clamp_i32(c: &Clamper, data: &[i32], lo: i32, hi: i32) -> Vec<i32> {
    spec! { c.clamp_all(data, lo, hi); Clamper; [i32, i32, i32] }
}

pub fn clamp_f64(c: &Clamper, data: &[f64], lo: f64, hi: f64) -> Vec<f64> {
    spec! { c.clamp_all(data, lo, hi); Clamper; [f64, f64, f64] }
}

pub fn clamp_u8(c: &Clamper, data: &[u8], lo: u8, hi: u8) -> Vec<u8> {
    spec! { c.clamp_all(data, lo, hi); Clamper; [u8, u8, u8] }
}

pub fn clamp_generic<T: Clone + PartialOrd>(c: &Clamper, data: &[T], lo: T, hi: T) -> Vec<T> {
    ClampTrait::clamp_all(c, data, lo, hi)
}
