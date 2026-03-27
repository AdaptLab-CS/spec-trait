// Example 14 — RUNTIME TYPEID: FMA
use std::any::TypeId;


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

