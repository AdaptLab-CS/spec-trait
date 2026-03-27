// Example 07 — RUNTIME TYPEID: Map / transform
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn map_runtime_i32(data: &[i32]) -> Vec<i32> {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        data.iter().map(|x| x.wrapping_mul(2)).collect()
    } else {
        data.to_vec()
    }
}

#[inline(never)]
pub fn map_runtime_f64(data: &[f64]) -> Vec<f64> {
    if TypeId::of::<f64>() == TypeId::of::<f64>() {
        data.iter().map(|x| x * 2.0).collect()
    } else {
        data.to_vec()
    }
}

