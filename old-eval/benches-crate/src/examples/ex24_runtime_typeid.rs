// Example 24 — RUNTIME TYPEID: T: 'static lifetime bound
use std::any::TypeId;


// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn hash_runtime_i32(data: &i32) -> u64 {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        let ptr = data as *const i32 as usize;
        (ptr as u64).wrapping_mul(0x517cc1b727220a95)
    } else {
        std::mem::size_of::<i32>() as u64
    }
}

