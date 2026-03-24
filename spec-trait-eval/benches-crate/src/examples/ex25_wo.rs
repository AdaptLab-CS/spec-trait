// Example 25 — WITHOUT macros: Conditional combinators (any, all, not)
use std::any::TypeId;

// --- enum dispatch ---
pub enum ProcessEnum<'a> {
    I32(&'a [i32]),
    I64(&'a [i64]),
    U8(&'a [u8]),
    F64(&'a [f64]),
    VecI32(&'a [Vec<i32>]),
}

#[inline(never)]
pub fn process_enum(variant: &ProcessEnum<'_>) -> usize {
    match variant {
        ProcessEnum::I32(data) => data.len() * 4,
        ProcessEnum::I64(data) => data.len() * 8,
        ProcessEnum::U8(data) => data.len(), // default for u8
        ProcessEnum::F64(data) => data.len() * 8,
        ProcessEnum::VecI32(data) => {
            let mut total = 0;
            for v in *data { total += v.len(); }
            total
        }
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn process_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() || TypeId::of::<i32>() == TypeId::of::<i64>() {
        data.len() * std::mem::size_of::<i32>()
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn process_runtime_u8(data: &[u8]) -> usize {
    // not(T = u8) doesn't match, so default
    data.len()
}

// --- generic fallback ---
#[inline(never)]
pub fn process_generic<T>(data: &[T]) -> usize {
    data.len()
}

// Bulk functions for benchmarking
#[inline(never)]
pub fn process_manual_i32(data: &[i32]) -> usize {
    data.len() * std::mem::size_of::<i32>()
}

#[inline(never)]
pub fn process_manual_u8(data: &[u8]) -> usize {
    data.len()
}

#[inline(never)]
pub fn process_manual_vec_i32(data: &[Vec<i32>]) -> usize {
    let mut total = 0;
    for v in data { total += v.len(); }
    total
}
