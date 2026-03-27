// Example 14 — GENERIC: FMA


// --- generic fallback ---
#[inline(never)]
pub fn fma_generic<T: Clone>(data: &[T], _mul: T, _add: T) -> Vec<T> {
    data.to_vec()
}
