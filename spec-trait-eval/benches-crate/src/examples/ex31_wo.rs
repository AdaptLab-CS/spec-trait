// Example 31 — WITHOUT macros: Many specializations (8+ types)
use std::any::TypeId;

// --- Approach A: enum dispatch ---
pub enum EncodeEnum<'a> {
    U8(&'a [u8]),
    I32(&'a [i32]),
    F64(&'a [f64]),
    U64(&'a [u64]),
}

#[inline(never)]
pub fn encode_enum(variant: &EncodeEnum) -> usize {
    match variant {
        EncodeEnum::U8(data) => data.len(),
        EncodeEnum::I32(data) => {
            let mut total = 0usize;
            for &v in *data {
                let uv = v as u32 as usize;
                total += if uv < 128 { 1 } else if uv < 16384 { 2 } else { 4 };
            }
            total
        }
        EncodeEnum::F64(data) => data.len() * 8,
        EncodeEnum::U64(data) => data.len() * 8,
    }
}

// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn encode_runtime_u8(data: &[u8]) -> usize {
    if TypeId::of::<u8>() == TypeId::of::<u8>() {
        data.len()
    } else {
        data.len() * std::mem::size_of::<u8>()
    }
}

#[inline(never)]
pub fn encode_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let mut total = 0usize;
        for &v in data {
            let uv = v as u32 as usize;
            total += if uv < 128 { 1 } else if uv < 16384 { 2 } else { 4 };
        }
        total
    } else {
        data.len() * 4
    }
}

// --- Approach C: generic fallback ---
#[inline(never)]
pub fn encode_generic_u8(data: &[u8]) -> usize {
    data.len() * std::mem::size_of::<u8>()
}

#[inline(never)]
pub fn encode_generic_i32(data: &[i32]) -> usize {
    data.len() * std::mem::size_of::<i32>()
}

// --- Approach D: manual specialization ---
#[inline(never)]
pub fn encode_manual_u8(data: &[u8]) -> usize {
    data.len()
}

#[inline(never)]
pub fn encode_manual_i32(data: &[i32]) -> usize {
    let mut total = 0usize;
    for &v in data {
        let uv = v as u32 as usize;
        total += if uv < 128 { 1 } else if uv < 16384 { 2 } else { 4 };
    }
    total
}

// --- Approach E: dyn dispatch ---
pub trait DynEncode {
    fn encode_dyn(&self) -> usize;
}

pub struct DynU8Encoder<'a>(pub &'a [u8]);
impl DynEncode for DynU8Encoder<'_> {
    #[inline(never)]
    fn encode_dyn(&self) -> usize {
        self.0.len()
    }
}

pub struct DynI32Encoder<'a>(pub &'a [i32]);
impl DynEncode for DynI32Encoder<'_> {
    #[inline(never)]
    fn encode_dyn(&self) -> usize {
        let mut total = 0usize;
        for &v in self.0 {
            let uv = v as u32 as usize;
            total += if uv < 128 { 1 } else if uv < 16384 { 2 } else { 4 };
        }
        total
    }
}

// --- Bulk functions ---
#[inline(never)]
pub fn sum_encode_enum_u8(chunks: &[&[u8]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        let e = EncodeEnum::U8(chunk);
        total = total.wrapping_add(encode_enum(&e));
    }
    total
}

#[inline(never)]
pub fn sum_encode_enum_i32(chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        let e = EncodeEnum::I32(chunk);
        total = total.wrapping_add(encode_enum(&e));
    }
    total
}

#[inline(never)]
pub fn sum_encode_runtime_i32(chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(encode_runtime_i32(chunk));
    }
    total
}

#[inline(never)]
pub fn sum_encode_generic_i32(chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(encode_generic_i32(chunk));
    }
    total
}

#[inline(never)]
pub fn sum_encode_manual_i32(chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        total = total.wrapping_add(encode_manual_i32(chunk));
    }
    total
}

#[inline(never)]
pub fn sum_encode_dyn_i32(chunks: &[&[i32]]) -> usize {
    let mut total = 0usize;
    for chunk in chunks {
        let d: &dyn DynEncode = &DynI32Encoder(chunk);
        total = total.wrapping_add(d.encode_dyn());
    }
    total
}
