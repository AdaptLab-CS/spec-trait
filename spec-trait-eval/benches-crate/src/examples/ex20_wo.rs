// Example 20 — WITHOUT macros: Dot product
use std::any::TypeId;

// --- enum ---
pub enum DotEnum<'a> {
    F64(&'a [f64], &'a [f64]),
    F32(&'a [f32], &'a [f32]),
    I32(&'a [i32], &'a [i32]),
}
#[inline(never)]
pub fn dot_enum_f64(variant: &DotEnum<'_>) -> f64 {
    match variant {
        DotEnum::F64(a, b) => a.iter().zip(b.iter()).map(|(x, y)| x * y).sum(),
        _ => 0.0,
    }
}

#[inline(never)]
pub fn dot_enum_f32(variant: &DotEnum<'_>) -> f32 {
    match variant {
        DotEnum::F32(a, b) => a.iter().zip(b.iter()).map(|(x, y)| x * y).sum(),
        _ => 0.0,
    }
}

#[inline(never)]
pub fn dot_enum_i32(variant: &DotEnum<'_>) -> i32 {
    match variant {
        DotEnum::I32(a, b) => a.iter().zip(b.iter()).map(|(x, y)| x.wrapping_mul(*y)).sum(),
        _ => 0,
    }
}

// --- runtime TypeId ---
#[inline(never)]
pub fn dot_runtime_f64(a: &[f64], b: &[f64]) -> f64 {
    if TypeId::of::<f64>() == TypeId::of::<f64>() {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    } else { 0.0 }
}

#[inline(never)]
pub fn dot_runtime_i32(a: &[i32], b: &[i32]) -> i32 {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        a.iter().zip(b.iter()).map(|(x, y)| x.wrapping_mul(*y)).sum()
    } else { 0 }
}

// --- generic fallback (no type knowledge → can't operate) ---
#[inline(never)]
pub fn dot_generic_f64(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).fold(0.0f64, |acc, (x, y)| acc + x * y)
}

#[inline(never)]
pub fn dot_generic_i32(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).fold(0i32, |acc, (x, y)| acc.wrapping_add(x.wrapping_mul(*y)))
}
