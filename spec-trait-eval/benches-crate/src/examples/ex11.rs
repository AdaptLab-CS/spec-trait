// Example 11 — WITH macros: Map / transform specialization
use spec_trait_macro::{spec, when};

pub struct Mapper;

pub trait MapTrait<T> {
    fn map_double(&self, data: &[T]) -> Vec<T>;
}

impl<T: Clone> MapTrait<T> for Mapper {
    #[inline(never)]
    fn map_double(&self, data: &[T]) -> Vec<T> {
        data.to_vec() // generic: identity
    }
}

#[when(T = i32)]
impl<T> MapTrait<T> for Mapper {
    #[inline(never)]
    fn map_double(&self, data: &[T]) -> Vec<T> {
        data.iter().map(|x| x.wrapping_mul(2)).collect()
    }
}

#[when(T = f64)]
impl<T> MapTrait<T> for Mapper {
    #[inline(never)]
    fn map_double(&self, data: &[T]) -> Vec<T> {
        data.iter().map(|x| x * 2.0).collect()
    }
}

#[when(T = u8)]
impl<T> MapTrait<T> for Mapper {
    #[inline(never)]
    fn map_double(&self, data: &[T]) -> Vec<T> {
        data.iter().map(|x| x.saturating_mul(2)).collect()
    }
}

pub fn map_i32(m: &Mapper, data: &[i32]) -> Vec<i32> {
    spec! { m.map_double(data); Mapper; [i32] }
}

pub fn map_f64(m: &Mapper, data: &[f64]) -> Vec<f64> {
    spec! { m.map_double(data); Mapper; [f64] }
}

pub fn map_u8(m: &Mapper, data: &[u8]) -> Vec<u8> {
    spec! { m.map_double(data); Mapper; [u8] }
}

pub fn map_generic<T: Clone>(m: &Mapper, data: &[T]) -> Vec<T> {
    MapTrait::map_double(m, data)
}
