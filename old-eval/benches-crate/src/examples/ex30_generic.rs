// Example 30 — GENERIC: any() predicate – 4-way integer fast path


// --- generic fallback ---
#[inline(never)]
pub fn filter_count_generic_i32(data: &[i32]) -> usize {
    data.len()
}

#[inline(never)]
pub fn filter_count_generic_u8(data: &[u8]) -> usize {
    data.len()
}

