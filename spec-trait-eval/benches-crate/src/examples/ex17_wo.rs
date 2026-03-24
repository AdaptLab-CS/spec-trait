// Example 17 — WITHOUT macros: Clamping
use std::any::TypeId;

// --- enum ---
pub enum ClampEnum<'a> {
    I32(&'a [i32], i32, i32),
    F64(&'a [f64], f64, f64),
    U8(&'a [u8], u8, u8),
}
#[inline(never)]
pub fn clamp_enum_i32(variant: &ClampEnum<'_>) -> Vec<i32> {
    match variant {
        ClampEnum::I32(data, lo, hi) => data.iter().map(|&x| x.clamp(*lo, *hi)).collect(),
        _ => vec![],
    }
}

#[inline(never)]
pub fn clamp_enum_f64(variant: &ClampEnum<'_>) -> Vec<f64> {
    match variant {
        ClampEnum::F64(data, lo, hi) => data.iter().map(|&x| x.clamp(*lo, *hi)).collect(),
        _ => vec![],
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn clamp_runtime_i32(data: &[i32], lo: i32, hi: i32) -> Vec<i32> {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        data.iter().map(|&x| x.clamp(lo, hi)).collect()
    } else {
        data.to_vec()
    }
}

// --- generic fallback ---
#[inline(never)]
pub fn clamp_generic<T: Clone + PartialOrd>(data: &[T], lo: T, hi: T) -> Vec<T> {
    data.iter().map(|x| {
        if *x < lo { lo.clone() }
        else if *x > hi { hi.clone() }
        else { x.clone() }
    }).collect()
}
