// Example 25 — GENERIC: &dyn Trait type pattern specialization


// --- Approach D: generic fallback ---
#[inline(never)]
pub fn render_generic_i32(_data: &i32) -> usize {
    std::mem::size_of::<i32>()
}

// --- Approach E: manual specialization ---
#[inline(never)]
pub fn render_manual_i32(_data: &i32) -> usize {
    4
}

