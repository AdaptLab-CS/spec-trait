// Example 35 — GENERIC: Two-generic concrete pair specialization


// --- generic fallback ---
#[inline(never)]
pub fn merge_count_generic_i32_u8(ts: &[i32], us: &[u8]) -> usize {
    ts.len() + us.len()
}

#[inline(never)]
pub fn merge_count_generic_i32_f64(ts: &[i32], us: &[f64]) -> usize {
    ts.len() + us.len()
}

