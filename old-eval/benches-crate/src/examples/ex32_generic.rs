// Example 32 — GENERIC: all() compound predicate with nested any()


// --- generic fallback ---
#[inline(never)]
pub fn fold_count_generic_i32(data: &[i32]) -> usize {
    data.len()
}

#[inline(never)]
pub fn fold_count_generic_u8(data: &[u8]) -> usize {
    data.len()
}

