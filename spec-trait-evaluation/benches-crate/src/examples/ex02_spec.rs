// Ex02 -- spec: to_bytes — Copy raw transmute vs Display formatting
use spec_trait_macro::{spec, when};
use std::fmt::Display;

pub struct Serializer;

pub trait ToByteTrait<T> {
    fn to_bytes(&self, data: &[T]) -> Vec<u8>;
}

impl<T: Display> ToByteTrait<T> for Serializer {
    #[inline(never)]
    fn to_bytes(&self, data: &[T]) -> Vec<u8> {
        let mut result = Vec::new();
        for item in data {
            result.extend_from_slice(format!("{}", item).as_bytes());
        }
        result
    }
}

#[when(T: Copy)]
impl<T> ToByteTrait<T> for Serializer {
    #[inline(never)]
    fn to_bytes(&self, data: &[T]) -> Vec<u8> {
        unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<T>(),
            )
            .to_vec()
        }
    }
}

pub fn to_bytes_u8(s: &Serializer, data: &[u8]) -> Vec<u8> {
    spec! { s.to_bytes(data); Serializer; [&[u8]]; u8: Copy }
}
