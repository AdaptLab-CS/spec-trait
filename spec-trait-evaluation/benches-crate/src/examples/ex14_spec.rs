// Ex14 -- spec: encode_val — negation and alternation dispatch
//
// Patterns: #[when(any(T = i32, T = i64))] and #[when(not(T = i32))]
// Logical combinators (any, not) for dispatch conditions cannot be expressed with
// enum dispatch, TypeId dispatch, or naive generics.
// Only spec-trait supports these logical dispatch patterns natively.
use spec_trait_macro::{spec, when};
use std::fmt::Debug;

pub struct Encoder;

pub trait EncodeValTrait<T> {
    fn encode_val(&self, data: &[T]) -> Vec<u8>;
}

impl<T: Debug> EncodeValTrait<T> for Encoder {
    #[inline(never)]
    fn encode_val(&self, data: &[T]) -> Vec<u8> {
        // Default: Debug-format each value (slow)
        let mut buf = Vec::new();
        for item in data {
            let s = format!("{:?}", item);
            buf.extend_from_slice(s.as_bytes());
        }
        buf
    }
}

#[when(any(T = i32, T = i64))]
impl<T> EncodeValTrait<T> for Encoder {
    #[inline(never)]
    fn encode_val(&self, data: &[T]) -> Vec<u8> {
        // Specialized: T is i32 or i64 — direct little-endian byte copy
        let byte_len = data.len() * std::mem::size_of::<T>();
        let mut buf = Vec::with_capacity(byte_len);
        unsafe {
            let src = data.as_ptr() as *const u8;
            buf.set_len(byte_len);
            std::ptr::copy_nonoverlapping(src, buf.as_mut_ptr(), byte_len);
        }
        buf
    }
}

pub fn encode_val_i32(enc: &Encoder, data: &[i32]) -> Vec<u8> {
    spec! { enc.encode_val(data); Encoder; [&[i32]] }
}
