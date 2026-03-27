// Example 07 — GENERIC: Map / transform


// --- generic fallback ---
#[inline(never)]
pub fn map_generic<T: Clone>(data: &[T]) -> Vec<T> {
    data.to_vec()
}
