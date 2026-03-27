// Example 18 — GENERIC: HRTB — higher-order function application


// --- generic fallback ---
#[inline(never)]
pub fn apply_generic<F: Fn(&i32) -> i32>(f: F, data: &[i32]) -> Vec<i32> {
    data.iter().map(|x| f(x)).collect()
}

