// Example 16 — GENERIC: Dot product


// --- generic fallback (no type knowledge → can't operate) ---
#[inline(never)]
pub fn dot_generic_f64(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).fold(0.0f64, |acc, (x, y)| acc + x * y)
}

#[inline(never)]
pub fn dot_generic_i32(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).fold(0i32, |acc, (x, y)| acc.wrapping_add(x.wrapping_mul(*y)))
}
