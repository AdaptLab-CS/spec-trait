// Example 19 — WITH macros: String conversion specialization
use spec_trait_macro::{spec, when};

pub struct Converter;

pub trait FromBytesTrait<T> {
    fn from_bytes(&self, data: &[u8]) -> Vec<T>;
}

impl<T> FromBytesTrait<T> for Converter {
    #[inline(never)]
    fn from_bytes(&self, _data: &[u8]) -> Vec<T> {
        vec![] // generic: can't convert
    }
}

#[when(T = u8)]
impl<T> FromBytesTrait<T> for Converter {
    #[inline(never)]
    fn from_bytes(&self, data: &[u8]) -> Vec<T> {
        data.to_vec()
    }
}

#[when(T = char)]
impl<T> FromBytesTrait<T> for Converter {
    #[inline(never)]
    fn from_bytes(&self, data: &[u8]) -> Vec<T> {
        data.iter().map(|&b| b as char).collect()
    }
}

#[when(T = i32)]
impl<T> FromBytesTrait<T> for Converter {
    #[inline(never)]
    fn from_bytes(&self, data: &[u8]) -> Vec<T> {
        data.chunks_exact(4)
            .map(|chunk| i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect()
    }
}

pub fn from_bytes_u8(c: &Converter, data: &[u8]) -> Vec<u8> {
    spec! { c.from_bytes(data); Converter; [u8] }
}

pub fn from_bytes_char(c: &Converter, data: &[u8]) -> Vec<char> {
    spec! { c.from_bytes(data); Converter; [char] }
}

pub fn from_bytes_i32(c: &Converter, data: &[u8]) -> Vec<i32> {
    spec! { c.from_bytes(data); Converter; [i32] }
}

pub fn from_bytes_generic<T>(c: &Converter, data: &[u8]) -> Vec<T> {
    FromBytesTrait::from_bytes(c, data)
}
