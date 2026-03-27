// Example 33 — GENERIC: Multiple receiver types (3 receivers, same trait)


// --- generic fallback ---
#[inline(never)]
pub fn count_generic_vec<T>(data: &[T]) -> usize {
    data.len()
}

#[inline(never)]
pub fn count_generic_slice<T>(data: &[T]) -> usize {
    data.len() * 2
}

#[inline(never)]
pub fn count_generic_array<T>(data: &[T]) -> usize {
    data.len() * 3
}

