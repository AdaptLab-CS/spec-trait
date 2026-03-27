// Example 31 — RUNTIME TYPEID: not() predicate – type exclusion
use std::any::TypeId;


// --- runtime TypeId ---
#[inline(never)]
pub fn compute_runtime_i32(data: &[i32]) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<String>() {
        data.len()
    } else {
        // not String → fast path
        data.len() * std::mem::size_of::<i32>()
    }
}

#[inline(never)]
pub fn compute_runtime_string(data: &[String]) -> usize {
    if TypeId::of::<String>() == TypeId::of::<String>() {
        let mut total = 0usize;
        for s in data {
            total += s.len();
        }
        total
    } else {
        data.len() * std::mem::size_of::<String>()
    }
}

