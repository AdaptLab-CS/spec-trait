// Example 21 — GENERIC: Conditional combinators (any, all, not)


// --- generic fallback ---
#[inline(never)]
pub fn process_generic<T>(data: &[T]) -> usize {
    data.len()
}

