// Example 27 — WITH macros: Many specializations (8+ when clauses)
// Stress test: a single trait with many type-specific dispatches.
use spec_trait_macro::{spec, when};

pub struct Encoder;

pub trait EncodeTrait<T> {
    fn encode_size(&self, data: &[T]) -> usize;
}

impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        data.len() * std::mem::size_of::<T>()
    }
}

#[when(T = u8)]
impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        // u8: 1 byte each, no overhead
        data.len()
    }
}

#[when(T = u16)]
impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        data.len() * 2
    }
}

#[when(T = u32)]
impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        data.len() * 4
    }
}

#[when(T = u64)]
impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        data.len() * 8
    }
}

#[when(T = i8)]
impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        data.len() + 1 // sign byte
    }
}

#[when(T = i16)]
impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        data.len() * 2 + 1
    }
}

#[when(T = i32)]
impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        // i32: varint encoding estimate
        let mut total = 0usize;
        for val in data {
            let v = val as *const T as usize;
            total += if v < 128 { 1 } else if v < 16384 { 2 } else { 4 };
        }
        total
    }
}

#[when(T = i64)]
impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        data.len() * 8 + 1
    }
}

#[when(T = f32)]
impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        data.len() * 4 // IEEE 754
    }
}

#[when(T = f64)]
impl<T> EncodeTrait<T> for Encoder {
    #[inline(never)]
    fn encode_size(&self, data: &[T]) -> usize {
        data.len() * 8 // IEEE 754
    }
}

pub fn encode_u8(e: &Encoder, data: &[u8]) -> usize {
    spec! { e.encode_size(data); Encoder; [&[u8]] }
}

pub fn encode_i32(e: &Encoder, data: &[i32]) -> usize {
    spec! { e.encode_size(data); Encoder; [&[i32]] }
}

pub fn encode_f64(e: &Encoder, data: &[f64]) -> usize {
    spec! { e.encode_size(data); Encoder; [&[f64]] }
}

pub fn encode_u64(e: &Encoder, data: &[u64]) -> usize {
    spec! { e.encode_size(data); Encoder; [&[u64]] }
}

pub fn encode_generic<T>(e: &Encoder, data: &[T]) -> usize {
    EncodeTrait::encode_size(e, data)
}

#[inline(never)]
pub fn sum_encode_u8(e: &Encoder, chunks: &[&[u8]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { e.encode_size(*chunk); Encoder; [&[u8]] });
    }
    total
}

#[inline(never)]
pub fn sum_encode_i32(e: &Encoder, chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(spec! { e.encode_size(*chunk); Encoder; [&[i32]] });
    }
    total
}

#[inline(never)]
pub fn sum_encode_generic<T>(e: &Encoder, chunks: &[&[T]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(EncodeTrait::encode_size(e, *chunk));
    }
    total
}
