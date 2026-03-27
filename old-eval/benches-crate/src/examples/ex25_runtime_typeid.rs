// Example 25 — RUNTIME TYPEID: &dyn Trait type pattern specialization
use std::any::TypeId;


// --- Approach B: runtime TypeId ---
#[inline(never)]
pub fn render_runtime_i32(_data: &i32) -> usize {
    if TypeId::of::<i32>() == TypeId::of::<i32>() {
        4
    } else {
        std::mem::size_of::<i32>()
    }
}

