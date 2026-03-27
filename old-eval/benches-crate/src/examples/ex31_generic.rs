// Example 31 — GENERIC: not() predicate – type exclusion


// --- generic fallback ---
#[inline(never)]
pub fn compute_generic_i32(data: &[i32]) -> usize {
    data.len()
}

#[inline(never)]
pub fn compute_generic_string(data: &[String]) -> usize {
    data.len()
}

