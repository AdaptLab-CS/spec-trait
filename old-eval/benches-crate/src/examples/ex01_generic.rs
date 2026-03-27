// Example 01 — GENERIC: Sum


// --- generic fallback (iter fold, no type knowledge) ---
#[inline(never)]
pub fn sum_generic_i32(data: &[i32]) -> i32 {
    data.iter().fold(0i32, |a, &b| a.wrapping_add(b))
}

#[inline(never)]
pub fn sum_generic_f64(data: &[f64]) -> f64 {
    data.iter().fold(0.0f64, |a, &b| a + b)
}
