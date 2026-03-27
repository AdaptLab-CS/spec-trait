// Example 01 — RUNTIME TYPEID: Sum
use std::any::TypeId;


// --- TypeId runtime check ---
#[inline(never)]
pub fn sum_runtime_i32(data: &[i32]) -> i32 {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        data.iter().copied().sum()
    } else {
        0
    }
}

#[inline(never)]
pub fn sum_runtime_f64(data: &[f64]) -> f64 {
    if TypeId::of::<f64>() == TypeId::of::<f64>() {
        data.iter().copied().sum()
    } else {
        0.0
    }
}

