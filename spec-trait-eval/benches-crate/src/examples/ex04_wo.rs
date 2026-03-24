// Example 4 — WITHOUT macros: Absolute value
use std::any::TypeId;

// --- dyn dispatch ---
pub trait DynAbs {
    fn abs_dyn(&self) -> Vec<u8>; // erased to bytes
}

// --- enum dispatch ---
pub enum AbsEnum<'a> {
    I32(&'a [i32]),
    F64(&'a [f64]),
    I64(&'a [i64]),
}

#[inline(never)]
pub fn abs_enum_all_i32(variant: &AbsEnum<'_>) -> Vec<i32> {
    match variant {
        AbsEnum::I32(d) => d.iter().map(|v| v.wrapping_abs()).collect(),
        _ => vec![],
    }
}

#[inline(never)]
pub fn abs_enum_all_f64(variant: &AbsEnum<'_>) -> Vec<f64> {
    match variant {
        AbsEnum::F64(d) => d.iter().map(|v| v.abs()).collect(),
        _ => vec![],
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn abs_runtime_i32(data: &[i32]) -> Vec<i32> {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        data.iter().map(|v| v.wrapping_abs()).collect()
    } else {
        data.to_vec()
    }
}

#[inline(never)]
pub fn abs_runtime_f64(data: &[f64]) -> Vec<f64> {
    if TypeId::of::<f64>() == TypeId::of::<f64>() {
        data.iter().map(|v| v.abs()).collect()
    } else {
        data.to_vec()
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn abs_generic_i32(data: &[i32]) -> Vec<i32> {
    data.to_vec()
}

#[inline(never)]
pub fn abs_generic_f64(data: &[f64]) -> Vec<f64> {
    data.to_vec()
}
