// Example 02 — GENERIC: Absolute value


// --- generic fallback ---
#[inline(never)]
pub fn abs_generic_i32(data: &[i32]) -> Vec<i32> {
    data.to_vec()
}

#[inline(never)]
pub fn abs_generic_f64(data: &[f64]) -> Vec<f64> {
    data.to_vec()
}
