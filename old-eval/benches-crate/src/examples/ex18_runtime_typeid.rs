// Example 18 — RUNTIME TYPEID: HRTB — higher-order function application
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn apply_runtime_i32(f: fn(&i32) -> i32, data: &[i32]) -> Vec<i32> {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        data.iter().map(|x| f(x)).collect()
    } else {
        data.iter().map(|x| f(x)).collect()
    }
}

