// Example 21 — RUNTIME TYPEID: Conditional combinators (any, all, not)
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn process_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() || TypeId::of::<i32>() == TypeId::of::<i64>() {
        data.len() * std::mem::size_of::<i32>()
    } else {
        data.len()
    }
}

#[inline(never)]
pub fn process_runtime_u8(data: &[u8]) -> usize {
    // not(T = u8) doesn't match, so default
    data.len()
}

