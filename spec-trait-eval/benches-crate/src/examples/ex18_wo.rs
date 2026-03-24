// Example 18 — WITHOUT macros: FMA
use std::any::TypeId;

// --- enum ---
pub enum FmaEnum<'a> {
    F64(&'a [f64], f64, f64),
    F32(&'a [f32], f32, f32),
    I32(&'a [i32], i32, i32),
}
#[inline(never)]
pub fn fma_enum_f64(variant: &FmaEnum<'_>) -> Vec<f64> {
    match variant {
        FmaEnum::F64(data, mul, add) => data.iter().map(|x| x.mul_add(*mul, *add)).collect(),
        _ => vec![],
    }
}

#[inline(never)]
pub fn fma_enum_i32(variant: &FmaEnum<'_>) -> Vec<i32> {
    match variant {
        FmaEnum::I32(data, mul, add) => data.iter().map(|x| x.wrapping_mul(*mul).wrapping_add(*add)).collect(),
        _ => vec![],
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn fma_runtime_f64(data: &[f64], mul: f64, add: f64) -> Vec<f64> {
    if TypeId::of::<f64>() == TypeId::of::<f64>() {
        data.iter().map(|x| x.mul_add(mul, add)).collect()
    } else {
        data.to_vec()
    }
}

#[inline(never)]
pub fn fma_runtime_i32(data: &[i32], mul: i32, add: i32) -> Vec<i32> {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        data.iter().map(|x| x.wrapping_mul(mul).wrapping_add(add)).collect()
    } else {
        data.to_vec()
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn fma_generic<T: Clone>(data: &[T], _mul: T, _add: T) -> Vec<T> {
    data.to_vec()
}
