// Example 28 — RUNTIME TYPEID: Tuple type specialization
use std::any::TypeId;


// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn squared_sum_runtime_i32(data: &[(i32, i32)]) -> f64 {
    if TypeId::of::<(i32, i32)>() == TypeId::of::<(i32, i32)>() {
        let mut sum = 0i64;
        for &(x, y) in data {
            sum += (x as i64) * (x as i64) + (y as i64) * (y as i64);
        }
        sum as f64
    } else {
        data.len() as f64
    }
}

#[inline(never)]
pub fn squared_sum_runtime_f64(data: &[(f64, f64)]) -> f64 {
    if TypeId::of::<(f64, f64)>() == TypeId::of::<(f64, f64)>() {
        let mut sum = 0.0f64;
        for &(x, y) in data {
            sum += x * x + y * y;
        }
        sum
    } else {
        data.len() as f64
    }
}

