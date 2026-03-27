// Example 37 — GENERIC: Generic receiver + type specialization


// --- generic fallback ---
#[inline(never)]
pub fn accum_count_generic_i32(data: &[i32]) -> usize {
    data.len()
}

#[inline(never)]
pub fn accum_count_generic_u8(data: &[u8]) -> usize {
    data.len()
}

