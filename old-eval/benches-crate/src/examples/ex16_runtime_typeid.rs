// Example 16 — RUNTIME TYPEID: Dot product
use std::any::TypeId;


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

