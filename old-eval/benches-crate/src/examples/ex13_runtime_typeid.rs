// Example 13 — RUNTIME TYPEID: Clamping
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn clamp_runtime_i32(data: &[i32], lo: i32, hi: i32) -> Vec<i32> {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        data.iter().map(|&x| x.clamp(lo, hi)).collect()
    } else {
        data.to_vec()
    }
}

