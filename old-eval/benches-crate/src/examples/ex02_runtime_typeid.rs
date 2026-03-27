// Example 02 — RUNTIME TYPEID: Absolute value
use std::any::TypeId;


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

