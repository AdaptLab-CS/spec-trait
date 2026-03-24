// Example 5 — WITH macros: Serialization (to bytes)
use spec_trait_macro::{spec, when};

pub struct Serializer;

pub trait SerTrait<T> {
    fn serialize(&self, val: &T) -> Vec<u8>;
}

impl<T: std::fmt::Debug> SerTrait<T> for Serializer {
    #[inline(never)]
    fn serialize(&self, val: &T) -> Vec<u8> {
        format!("{:?}", val).into_bytes()
    }
}

#[when(T = i32)]
impl<T> SerTrait<T> for Serializer {
    #[inline(never)]
    fn serialize(&self, val: &T) -> Vec<u8> {
        val.to_le_bytes().to_vec()
    }
}

#[when(T = f64)]
impl<T> SerTrait<T> for Serializer {
    #[inline(never)]
    fn serialize(&self, val: &T) -> Vec<u8> {
        val.to_le_bytes().to_vec()
    }
}

#[when(T = String)]
impl<T> SerTrait<T> for Serializer {
    #[inline(never)]
    fn serialize(&self, val: &T) -> Vec<u8> {
        let bytes = val.as_bytes();
        let mut out = Vec::with_capacity(4 + bytes.len());
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(bytes);
        out
    }
}

pub fn ser_i32(s: &Serializer, val: &i32) -> Vec<u8> {
    spec! { s.serialize(val); Serializer; [i32] }
}

pub fn ser_f64(s: &Serializer, val: &f64) -> Vec<u8> {
    spec! { s.serialize(val); Serializer; [f64] }
}

pub fn ser_string(s: &Serializer, val: &String) -> Vec<u8> {
    spec! { s.serialize(val); Serializer; [String] }
}

pub fn ser_generic<T: std::fmt::Debug>(s: &Serializer, val: &T) -> Vec<u8> {
    SerTrait::serialize(s, val)
}
